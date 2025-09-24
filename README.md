A blazing🔥 fast command-line💻 tool to find🔍 a given input in a file🗒️ or directory📁.

Example usage: `grrs <input> [<location>]`

### How to install:
- Clone this repo with `git clone https://github.com/TFerd/grrs.git`
- Run `cargo build --release`

___

TODO:
- [x] fix verbosity
- [x] add color highlighting to matches in files
- [x] add file name where the phrase was found
- [x] move reusable code into function
- [x] fix --output flag recursion bug
- [x] add threading
- [x] try rayon crate for threads?
- [ ] change `for i in dir_entries` to an `into_iter().for_each()` loop, i think it's better for memory? (un)educated guess <- memory? what??? 
- [ ] pass around an options or configuration struct to handle things like verbosity and output flags etc
- [ ] implement the `-help` and `--help` flags
- [ ] add regex?
- [ ] add fuzzy search?
