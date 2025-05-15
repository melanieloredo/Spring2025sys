// Bonus Features: --period <sec>, summary statistics, --assert-header <Header:Value> added
use std::{
    env,                    //CLI arguments
    fs::File,               //for reading files
    io::{self, BufRead},    //file lines
    path::Path,             //file paths
    sync::Arc,
    thread,                 //spawning threads
    time::{Duration, Instant, SystemTime}, //timing and timestamps
};

//import reqwest HTTP client (blocking version)
use reqwest::blocking::{Client, ClientBuilder};
use std::fs;
use std::sync::Mutex; //for thread-safe shared data

//Ownership & borrowing across threads | Sharing configuration and sending data structures safely
//Clone is required because WebsiteStatus is shared across threads and needs to be cloned safely.
#[derive(Debug, Clone)] //for sharing results between threads

//structure for 1 check
struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>, //http status as a string
    response_time: Duration,            //request length
    timestamp: SystemTime,              //time of check
    header_valid: bool,
}

//argument parsed, reading the file, and grabbing the URL
fn parse_args() -> (Vec<String>, usize, u64, u32, Option<u64>, Option<(String, String)>) {
    let args: Vec<String> = env::args().collect();

    let mut urls = Vec::new();
    let mut file_path = None;
    let mut workers = num_cpus::get(); //use all CPU cores
    let mut timeout = 5;                //Default timeout in seconds
    let mut retries = 0;                //no retries
    let mut period = None;
    let mut assert_header = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i < args.len() {
                    file_path = Some(args[i].clone());
                }
            }
            "--workers" => {
                i += 1;
                if i < args.len() {
                    workers = args[i].parse().unwrap_or(workers);
                }
            }
            "--timeout" => {
                i += 1;
                if i < args.len() {
                    timeout = args[i].parse().unwrap_or(timeout);
                }
            }
            "--retries" => {
                i += 1;
                if i < args.len() {
                    retries = args[i].parse().unwrap_or(retries);
                }
            }
            "--period" => {
                i += 1;
                if i < args.len() {
                    period = args[i].parse().ok();
                }
            }
            "--assert-header" => {
                i += 1;
                if i < args.len() && args[i].contains(":") {
                    let parts: Vec<&str> = args[i].splitn(2, ':').collect();
                    assert_header = Some((parts[0].trim().to_string(), parts[1].trim().to_string()));
                }
            }
            _ => {
                if !args[i].starts_with("--") {
                    urls.push(args[i].clone()); //add plain URLs
                }
            }
        }
        i += 1;
    }

    //If file provided, read it line-by-line into the URL list
    if let Some(file) = file_path {
        if let Ok(lines) = read_lines(&file) {
            for line in lines.flatten() {
                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }
                urls.push(line);
            }
        }
    }
    //If no URLs provided, show usage and exit
    if urls.is_empty() {
        eprintln!("Please use the format: cargo run -- [--file filename.txt] [URL ...] [--workers N] [--timeout S] [--retries N] [--period S] [--assert-header Header:Value]");
        std::process::exit(2);
    }

    (urls, workers, timeout, retries, period, assert_header)
}

//function to read lines from a file and returns an iterator
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//function to check a single website and return status info
fn check_website(client: &Client, url: &str, timeout: Duration, retries: u32, header_check: &Option<(String, String)>) -> WebsiteStatus {
    let mut attempts = 0;
    //Measuring latency | Using std::time::Instant
    let start = Instant::now();
    let timestamp = SystemTime::now();
    let result;
    let mut header_valid = true; //header

    loop {
        //External crate integration | Using a single HTTP client crate (reqwest) correctly
        let response = client.get(url).timeout(timeout).send();

        //Error handling | Returning Result<_, String> without panics
        match response {
            Ok(resp) => {
                if let Some((key, expected)) = header_check {
                    header_valid = resp.headers().get(key).map_or(false, |v| v.to_str().unwrap_or("") == expected);
                }
                result = Ok(resp.status().as_u16());
                break;
            }
            Err(e) => {
                attempts += 1;
                if attempts > retries {
                    result = Err(e.to_string());
                    header_valid = false;
                    break;
                }
                thread::sleep(Duration::from_millis(100)); //wait before retry
            }
        }
    }

    //return a record of the result
    WebsiteStatus {
        url: url.to_string(),
        action_status: result,
        response_time: start.elapsed(),
        timestamp,
        header_valid,
    }
}

fn run_once(urls: Vec<String>, workers: usize, timeout: Duration, retries: u32, header_check: &Option<(String, String)>) -> Vec<WebsiteStatus> {
    let client = Arc::new(ClientBuilder::new().timeout(timeout).build().unwrap());
    let urls = Arc::new(Mutex::new(urls));
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for _ in 0..workers {
        let urls = Arc::clone(&urls);
        let client = Arc::clone(&client);
        let results = Arc::clone(&results);
        let header_check = header_check.clone();

        let handle = thread::spawn(move || {
            loop {
                let url = {
                    let mut locked = urls.lock().unwrap();
                    if locked.is_empty() {
                        break;
                    }
                    locked.pop()
                };

                if let Some(url) = url {
                    let status = check_website(&client, &url, timeout, retries, &header_check);
                    {
                        results.lock().unwrap().push(status.clone());
                    }
                    let display = match &status.action_status {
                        Ok(code) => format!("{} OK [{}] {:?} {}", status.url, code, status.response_time, if status.header_valid { "" } else { "[Header Mismatch]" }),
                        Err(e) => format!("{} ERROR [{}] {:?}", status.url, e, status.response_time),
                    };
                    println!("{}", display);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

fn print_summary(results: &[WebsiteStatus]) {
    let times: Vec<u128> = results.iter().map(|r| r.response_time.as_millis()).collect();
    if times.is_empty() {
        return;
    }
    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();
    let avg = times.iter().sum::<u128>() as f64 / times.len() as f64;
    println!("\nSummary: Min = {}ms, Max = {}ms, Avg = {:.2}ms\n", min, max, avg);
}

fn write_json(results: &[WebsiteStatus]) {
    let json_array: Vec<String> = results.iter().map(|s| {
        let status = match &s.action_status {
            Ok(code) => format!("\"status\": {}", code),
            Err(e) => format!("\"error\": \"{}\"", e.replace('"', "\\\"")),
        };
        format!(
            "{{\"url\": \"{}\", {}, \"response_time_ms\": {}, \"timestamp\": {}, \"header_valid\": {}}}",
            s.url,
            status,
            s.response_time.as_millis(),
            s.timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            s.header_valid
        )
    }).collect();

    let json_output = format!("[\n{}\n]", json_array.join(",\n"));
    fs::write("status.json", json_output).expect("Unable to write JSON file");
}

fn main() {
    let (original_urls, workers, timeout_sec, retries, period, header_check) = parse_args();
    let timeout = Duration::from_secs(timeout_sec);

    loop {
        let urls = original_urls.clone();
        let results = run_once(urls, workers, timeout, retries, &header_check);
        print_summary(&results);
        write_json(&results);

        if let Some(pause) = period {
            println!("Sleeping for {} seconds before next check...", pause);
            thread::sleep(Duration::from_secs(pause));
        } else {
            break;
        }
    }
}

