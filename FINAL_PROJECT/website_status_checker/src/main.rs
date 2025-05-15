use std::{
    env,                    //CLI arguments
    fs::File,               //for reading files
    io::{self, BufRead},    //file lines
    path::Path,             //file paths
    sync::{mpsc, Arc},      //for channels and shared ownership
    thread,                 //spawning threads
    time::{Duration, Instant, SystemTime}, //timing and timestamps
};

//import reqwest HTTP client (blocking version)
use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
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
}

//argument parsed, reading the file, and grabbing the URL
fn parse_args() -> (Vec<String>, usize, u64, u32) {
    let args: Vec<String> = env::args().collect();

    let mut urls = Vec::new();
    let mut file_path = None;
    let mut workers = num_cpus::get(); //use all CPU cores
    let mut timeout = 5;                //Default timeout in seconds
    let mut retries = 0;                //no retries

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
        eprintln!("Please use the format: cargo run -- [--file filename.txt] [URL ...] [--workers N] [--timeout S] [--retries N]");
        std::process::exit(2);
    }

    (urls, workers, timeout, retries)
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
fn check_website(client: &Client, url: &str, timeout: Duration, retries: u32) -> WebsiteStatus {
    let mut attempts = 0;
    //Measuring latency | Using std::time::Instant
    let start = Instant::now();
    let timestamp = SystemTime::now();
    let result;

    loop {
        //External crate integration | Using a single HTTP client crate (reqwest) correctly
        let response = client.get(url).timeout(timeout).send();

        //Error handling | Returning Result<_, String> without panics
        match response {
            Ok(resp) => {
                result = Ok(resp.status().as_u16());
                break;
            }
            Err(e) => {
                attempts += 1;
                if attempts > retries {
                    result = Err(e.to_string());
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
    }
}

fn main() {

    //get user input to parse
    let (urls, workers, timeout_sec, retries) = parse_args();
    let timeout = Duration::from_secs(timeout_sec);
    //External crate integration | Using reqwest::blocking::Client
    let client = Arc::new(ClientBuilder::new().timeout(timeout).build().unwrap());

    //Thread creation & coordination | Worker-thread pool pulling jobs from a channel
    let (tx, rx) = mpsc::channel(); //channel for completion channels

    //share URL queue and result storage in between threads with mutexes
    let urls = Arc::new(Mutex::new(urls)); 
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for _ in 0..workers {
        //Ownership & borrowing across threads | Arc is used to safely share data
        let urls = Arc::clone(&urls); //clone shared state for the thread
        let tx = tx.clone();
        let client = Arc::clone(&client);
        let results = Arc::clone(&results);

        //Thread creation | Worker thread loop
        let handle = thread::spawn(move || { //spawn the worker thread
            loop {
                //next URL to check
                let url = {
                    let mut locked = urls.lock().unwrap();
                    if locked.is_empty() {
                        break;
                    }
                    locked.pop()
                };

                if let Some(url) = url {
                    //check website status
                    let status = check_website(&client, &url, timeout, retries);
                    //Ownership & borrowing across threads | Mutex protects shared result vec
                    {
                        //save result
                        results.lock().unwrap().push(status.clone());
                    }
                    //output the status
                    let display = match &status.action_status {
                        Ok(code) => format!("{} OK [{}] {:?}", status.url, code, status.response_time),
                        Err(e) => format!("{} ERROR [{}] {:?}", status.url, e, status.response_time),
                    };
                    println!("{}", display);

                    //signal that thread is completed
                    tx.send(()).unwrap();
                }
            }
        });
        handles.push(handle);
    }

    //wait for threads (workers) to complete
    for _ in 0..results.lock().unwrap().len() {
        let _ = rx.recv();
    }
    //join all threads for clean exit
    for handle in handles {
        let _ = handle.join();
    }

    // Simple JSON generation | Building a JSON file manually, no helper crates
    let json_array: Vec<String> = results.lock().unwrap().iter().map(|s| {
        let status = match &s.action_status {
            Ok(code) => format!("\"status\": {}", code),
            Err(e) => format!("\"error\": \"{}\"", e.replace('"', "\\\"")),
        };
        format!(
            "{{\"url\": \"{}\", {}, \"response_time_ms\": {}, \"timestamp\": {:?}}}",
            s.url,
            status,
            s.response_time.as_millis(),
            s.timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
        )
    }).collect();

    //write JSON array
    let json_output = format!("[\n{}\n]", json_array.join(",\n"));
    fs::write("status.json", json_output).expect("Unable to write JSON file");
}

