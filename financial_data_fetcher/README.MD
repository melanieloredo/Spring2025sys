# 📈 Financial Data Fetcher Assignment

## 🧠 Features

- Fetches real-time prices for:
  - **Bitcoin (BTC)** from CoinGecko  
  - **Ethereum (ETH)** from CoinGecko  
  - **S&P 500** (simulated value)
- Saves each coin data into a timestamped CSV file (e.g., `bitcoin_price.csv`)
- Repeats every 10 seconds

---

## 📦 Dependencies

- [`ureq`](https://docs.rs/ureq/latest/ureq/): for making HTTP requests  
- [`serde`](https://serde.rs/): for JSON parsing  
- [`chrono`](https://docs.rs/chrono/latest/chrono/): for timestamping  
- Standard library (`std`) for I/O and threading

---

## 🛠️ How It Works

### Traits

Involves a `Pricing` trait with the methods:
- `get_price()` - which returns the current price  
- `fetch_price()` - fetches the updated price from API or simulation  
- `save_to_file(filename)` - saves the price with timestamp to a CSV  
- `get_name()` - returns the coin's name  

### Structure Implementations

- `Bitcoin`, `Ethereum`, and `SP500` structs each implement `Pricing`.
- `SP500` is simulated with a static price because I could not find SP500 API for free.

### Main loop (simplified)

```rust
loop {
    for asset in assets.iter_mut() {
        asset.fetch_price();     // pull price from source
        asset.save_to_file();    // save to file
    }

    thread::sleep(Duration::from_secs(10)); // repeat every 10 seconds
}
```

---

## ▶️ How to run

1. **With Rust installed**,
**Install dependenceis and run the program**:
   ```bash
   cargo add ureq serde serde_json chrono
   cd financial_data_fetcher
   cargo run
   ```

2. **output shows updates on what is currently running**:
   - console will log current process.
   - CSV files (`bitcoin_price.csv`, `ethereum_price.csv`, `s&p_500_price.csv`) will be created/appended in the current `financial_data_fetcher` folder.

---

## 🗂 Example of File Output

```csv
2025-04-11 10:30:12.123456Z, Bitcoin, 71523.89
2025-04-11 10:30:12.123456Z, Ethereum, 3712.42
2025-04-11 10:30:12.123456Z, S&P 500, 4000.00
```

---
