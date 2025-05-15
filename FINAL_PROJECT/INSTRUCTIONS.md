Final Project Assignment: Website Status Checker (Rust)
---------------------------------------------

1) **Project Overview**

Design and implement a concurrent website‑monitoring tool that can check the availability of many websites in parallel. The finished program will be a command‑line utility written entirely in Rust.

*Purpose*: reinforce your understanding of Rust threads, channels, error handling, external‐process management, and simple CLI design.

---------------------------------------------
2) **Learning Goals**

**Concept** _________________	|   __________	        **Demonstrated By**

Thread creation & coordination	 ___ |____ Fixed worker‑thread pool that pulls jobs from a channel

Ownership & borrowing across threads __ | ___	Sharing configuration and sending data structures safely

Error handling __ | ___	Returning Result<_, String> without panics

Measuring latency __ | ___	Using std::time::Instant

Simple JSON generation __ | ___	Building a JSON file manually, no helper crates

External crate integration __ | ___	Using a single HTTP client crate (reqwest) correctly

---------------------------------------------

3)  **Functional Requirements**
    1) Input sources

        File: --file <path> (text file, one URL per line; blank lines & lines starting with # are ignored).

        Positional arguments: any URLs given directly after flags.

    2) Concurrency

        A fixed pool of worker threads (--workers N, default = number of logical CPU cores).
    3) Timeout
        Per‑request timeout (--timeout <seconds>, default = 5 s) enforced via the HTTP client builder.
    4) Retries
        Optional --retries <N> (default = 0) additional attempts after a failure, with a 100 ms pause between attempts.
    5) Result capture
        For each URL collect: HTTP status code or error string, total response time, and a timestamp.
    6) Live output
        Immediately print one human‑readable line per URL to stdout.
    7) Batch output
        After all URLs finish, write status.json containing an array of objects with the same data.
---------------------------------------------

3.1 WebsiteStatus Structure
---------------------------------------------
url: String                // original URL

action_status: Result<u16, String> // HTTP code or error text

response_time: Duration    // how long the request took

timestamp: SystemTime      // when the attempt completed

(Field names in JSON can be your choice, but must be documented.)

---------------------------------------------

4  Technical Constraints
---------------------------------------------
- Allowed third‑party crates: only one—an HTTP client  reqwest with the blocking feature. No other crates are permitted.
- No unsafe code.
- Use only the standard library and reqwest; everything else must be hand‑written.
- No custom signal handling is required; let the OS kill the program with Ctrl‑C if desired.

---------------------------------------------

5  Command‑Line Syntax
---------------------------------------------
website_checker [--file sites.txt] [URL ...]
               [--workers N] [--timeout S] [--retries N]
If neither --file nor positional URLs are supplied, print a helpful usage message and exit with code 2.

---------------------------------------------

6  Bonus Features (Optional)
---------------------------------------------
Bonus	Suggested Implementation Hint

--period <sec>	Loop forever, sleeping the given interval between rounds.

Summary statistics	Track min / max / average response times and print after each round.

HTTP header assertions	Add a flag to check for a specific header value.

---------------------------------------------

7  Deliverables & Submission
---------------------------------------------
Item	Notes

GitHub repository	Public, named website-status-checker-rust (or similar). Must compile on stable Rust ≥ 1.78.

Source layout	Standard Cargo project (Cargo.toml, src/main.rs, plus any modules).

README.md	~1 page: build instructions (cargo build --release), usage examples, and explanation of any bonus features.

screenshot.png	Terminal screenshot of a sample run showing at least one successful and one failed URL.

Submit the GitHub URL through the BlackBoard by the deadline.