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
    fn save_to_file(&self, filename: &str);
    fn get_name(&self) -> &str;
    fn get_price(&self) -> f64;
}

//implementation of pricing for each
impl Pricing for Bitcoin{
    fn get_price(&self) -> f64 {
        self.pricing
    }
}

impl Pricing for Ethereum{
    fn get_price(&self) -> f64 {
        self.pricing
    }
}

impl Pricing for SP500{
    fn get_price(&self) -> f64 {
        self.pricing
    }
}

fn main() {

}
