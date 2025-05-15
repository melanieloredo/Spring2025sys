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
