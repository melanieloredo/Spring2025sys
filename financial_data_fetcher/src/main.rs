//libraries
use ureq;
use serde::Deserialize;
use std::{fs::File, io::{self, Write}, thread, time::Duration};
use chrono::Utc;


//to Deserialize API Response
#[derive(Debug, Deserialize)]
struct CoinGeckoResponse {
    bitcoin: Option<CoinPrice>,
    ethereum: Option<CoinPrice>,
}

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
    fn get_price(&self) -> f64;
    fn save_to_file(&self, filename: &str) -> Result<(), io::Error>;
    fn get_name(&self) -> &str;
    fn fetch_price(&mut self) -> Result<(), String>;
}

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
    fn get_price(&self) -> f64 {
        self.pricing
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

        let response: serde_json::Value = ureq::get(url)
            .call()
            .map_err(|e| format!("HTTP error: {}", e))?
            .into_json()
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        if let Some(price) = response["bitcoin"]["usd"].as_f64() {
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
        let content = format!("{},{},{}\n", timestamp, self.name, self.pricing);
        file.write_all(content.as_bytes())
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
        let content = format!("{},{},{}\n", timestamp, self.name, self.pricing);
        file.write_all(content.as_bytes())
    }
}//end of pricing for eth

impl SP500 {
    fn new() -> Self {
        SP500 {
            pricing: 4000.0, // simulated static value
            name: "S&P 500".to_string(),
        }
    }
}
//pricing for sp500 is static because was not part of coingecko 
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
        let content = format!("{},{},{}\n", timestamp, self.name, self.pricing);
        file.write_all(content.as_bytes())
    }
}

fn main() {

    // Initialization with assets
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SP500::new()),
    ];

    loop {
        println!("--- Fetching and saving prices ---");

        for asset in assets.iter_mut() {
            if let Err(err) = asset.fetch_price() {
                eprintln!("Error fetching {}: {}", asset.get_name(), err);
                continue;
            }

            let filename = format!("{}_price.csv", asset.get_name().to_lowercase().replace(' ', "_"));
            if let Err(err) = asset.save_to_file(&filename) {
                eprintln!("Error saving {}: {}", asset.get_name(), err);
            }
        }

        println!("Waiting 10 seconds...\n");
        thread::sleep(Duration::from_secs(10));
    }
}
