# LogLyzer

Log analyzer for JSONL format

## Installation

1. Install Rust programming language any way you prefer (refer to the docs https://www.rust-lang.org/tools/install)
2. Get repository (```git clone https://github.com/igorslepenkov/loglyzer.git```)
3. Cd into repo
4. Run ```cargo build --release```
5. Move binary from target/release/loglyzer to somewhere in your PATH
6. Run ``` loglyzer -h ```

## Usage

```loglyzer -d "/home/venderart/projects/usefull" --filter "requestId =='rmq-no-context-1wt75jh'" --sort-by 'time'```

## Notes

1) Supports only JSONL format
2) Currently supports only .log and .log.gz
3) Filter and sort_by should be valid JMESPath expressions
