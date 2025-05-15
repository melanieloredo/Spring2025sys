~1 page: build instructions (cargo build --release), usage examples, and explanation of any bonus features.

**Different ways to run the checker**
-------

1. cargo run -- --file urls.txt

*Purpose*: All URLs within the text file are added to be checked.

2. cargo run -- https://google.com

*Purpose*: Added to the list of sites to check even without a file.

3. cargo run -- --file urls.txt --workers 4

*Purpose*: Control how many threads run concurrently.

4. cargo run -- --timeout 3 https://google.com

*Purpose*: How long to wait before timing out.

5. cargo run -- --file urls.txt --retries 2

*Purpose*: How many times to rety connecting to site.

6. cargo run -- --file urls.txt --workers 6 --timeout 3 --retries 1

*Purpose*: This one does them all at once

-------
**Bonus Features**
-------
1. cargo run -- --file urls.txt --period 60

*Purpose*: Runs the website checker in an infinite loop, sleeping for seconds between rounds.

2. Summary Statistics

> Summary: Min = 48ms, Max = 312ms, Avg = 117.20ms

*Purpose*: Runs automatically after every round, giving a summary

3. cargo run -- --file sites.txt --assert-header Server:cloudflare

*Purpose*: Checks that a specific HTTP header equals a given value for each response.