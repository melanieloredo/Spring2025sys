//libraries
use ureq; //http & get requests
use serde::Deserialize; //for JSON in a rust struct
use std::{fs::File, io::{self, Write}, thread, time::Duration}; //writing data to file & delay loop
use chrono::Utc; //to get the timestamp


//to deserialize API Response
#[derive(Debug, Deserialize)]

struct CoinGeckoResponse {
    bitcoin: Option<CoinPrice>,
    ethereum: Option<CoinPrice>,
}
//format price passed from API
#[derive(Debug, Deserialize)]
struct CoinPrice {
    usd: f64,
}

//Bitcoin, Ethereum and SP500 struct
struct Bitcoin{
    pricing: f64,
    name: String,
}

struct Ethereum{
    pricing: f64,
    name: String,
}

struct SP500{
    pricing: f64,
    name: String,
}

//trait pricing
trait Pricing {
    fn get_price(&self) -> f64; //get price
    fn save_to_file(&self, filename: &str) -> Result<(), io::Error>; //save to file
    fn get_name(&self) -> &str; //get name of coin (for file output)
    fn fetch_price(&mut self) -> Result<(), String>; //update price
}
//new() constructor for implementing pricing
impl Bitcoin {
    fn new() -> Self {
        Bitcoin {
            pricing: 0.0,
            name: "Bitcoin".to_string(),
        }
    }
}

//implementation of pricing for Bitcoin
impl Pricing for Bitcoin{

    //get the current price
    fn get_price(&self) -> f64 {
        self.pricing
    }
    //get the reference of the coin's name
    fn get_name(&self) -> &str {
        &self.name
    }
    //updating price from api
    fn fetch_price(&mut self) -> Result<(), String> {
        //url that returns the price
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

        let response: serde_json::Value = ureq::get(url) //GET
            .call()                                      //perform a call to the request
            .map_err(|e| format!("HTTP error: {}", e))? //if http error return error, if success return the unwrap result (catch error)
            .into_json()                                //converts HTTP to JSON
            .map_err(|e| format!("Failed to parse JSON: {}", e))?; //if http error return error, if success continue (catch error)

        if let Some(price) = response["bitcoin"]["usd"].as_f64() { //if price exists and can be found,
            self.pricing = price; //update with new price
            println!("Fetched {} price: ${}", self.name, self.pricing); //print to console
            Ok(()) //if successful
        } else { //else catch error
            Err("price not found in response".to_string())
        }
    }
    //receives a reference to current instance & borrowed string, returns if successful and any errors
    fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut file = File::options().append(true).create(true).open(filename)?; //creating file if doesnt exist, apending and opening file, returns result or error(catch error)
        let timestamp = Utc::now(); //grabs current time
        let content = format!("{}, {}, {}\n", timestamp, self.name, self.pricing); //format of each line
        file.write_all(content.as_bytes()) //converts to bytes
    }
}

 //end of bitcoin implementation

impl Ethereum {
    fn new() -> Self {
        Ethereum {
            pricing: 0.0,
            name: "Ethereum".to_string(),
        }
    }
}

impl Pricing for Ethereum{
    fn get_price(&self) -> f64 {
        self.pricing
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";

        let response: serde_json::Value = ureq::get(url)
            .call()
            .map_err(|e| format!("HTTP error: {}", e))?
            .into_json()
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        if let Some(price) = response["ethereum"]["usd"].as_f64() {
            self.pricing = price;
            println!("Fetched {} price: ${}", self.name, self.pricing);
            Ok(())
        } else {
            Err("Price not found in response".to_string())
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut file = File::options().append(true).create(true).open(filename)?;
        let timestamp = Utc::now();
        let content = format!("{}, {}, {}\n", timestamp, self.name, self.pricing);
        file.write_all(content.as_bytes())
    }
}//end of pricing for eth

//constructor w fixed pricing
// pricing for sp500 is static because was not part of coingecko 
// also could not be found free
impl SP500 {
    fn new() -> Self {
        SP500 {
            pricing: 4000.0, // simulated static value
            name: "S&P 500".to_string(),
        }
    }
}
//implementation for pricing
impl Pricing for SP500{
    fn get_price(&self) -> f64 {
        self.pricing
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn fetch_price(&mut self) -> Result<(), String> {
        println!("(Simulated) Fetched {} price: ${}", self.name, self.pricing);
        Ok(())
    }

    fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut file = File::options().append(true).create(true).open(filename)?;
        let timestamp = Utc::now();
        let content = format!("{}, {}, {}\n", timestamp, self.name, self.pricing);
        file.write_all(content.as_bytes())
    }
}

fn main() {

    // initialization vector list with assets //wrapped so multiple can exist
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SP500::new()),
    ];

    //infinite loop
    loop {
        println!("--- Fetching and Saving prices ---");

        //iterate through each asset
        for asset in assets.iter_mut() {
            //if error fetching return the error
            if let Err(err) = asset.fetch_price() {
                eprintln!("Error fetching {}: {}", asset.get_name(), err);
                continue;
            }
            //creating files
            let filename = format!("{}_price.csv", asset.get_name().to_lowercase().replace(' ', "_"));
            //if error with files return the error
            if let Err(err) = asset.save_to_file(&filename) {
                eprintln!("Error saving {}: {}", asset.get_name(), err);
            }
        }
        //indicate next 10 seconds
        println!("Waiting 10 seconds...\n");

        //pause program for 10 secs and repeats the infinite loop
        thread::sleep(Duration::from_secs(10));
    }
}
