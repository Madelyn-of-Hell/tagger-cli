
# TAGGER-CLI
The Command-Line Interface for an as-yet unnamed tag-based file manager.
Developed as part of Hackclub Stardance. Tested on MacOS, but theoretically compatible with Linux and Windows.  
![Uptime Badge](https://hackatime.hackclub.com/api/v1/badge/U09PRS9CZNJ/tagger)
## Installation
Clone the repo, build it, then chuck it somewhere your path will find it.
e.g. (For MacOS)
```zsh
git clone https://github.com/Madelyn-of-Hell/tagger-cli.git
cd tagger-cli
cargo build --release
mv target/debug/tagger /usr/local/bin/tagger
```

## Testing
As the tests require accessing the same repo and modifying/wiping the directory, it's recommended to force them to run sequentially so they can't interfere with one another.
```zsh
cargo test -- --test-threads 1
```
