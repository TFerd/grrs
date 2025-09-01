A 🔥blazing fast 💻command-line tool to 🔍find a given input in a 🗒️file or 📁directory.

Example usage: `grrs <input> [<location>]`

[reference](https://rust-cli.github.io/book/)


TODO:
- [x] fix verbosity (get rid of if else hell)
- [x] add color highlighting to matches in files
- [x] add file name where the phrase was found
- ~~[ ] do this in C~~
- ~~[ ] move reusable code into function (probably the stuff with output flag)~~
- [x] fix --output flag recursion bug (i just needed a newline lmfao)
- ~~[ ] add installation process to Cargo.toml(?)~~
- [x] add threading
- [ ] change `for i in dir_entries` to an `into_iter().for_each()` loop, i think it's better for memory? (un)educated guess
- [x] try rayon crate for threads?
- [ ] check the output file clone is not too heavy and make sure its working
- [ ] pass around an options or configuration struct to handle things like verbosity and output flags etc
- [ ] implement the `-help` and `--help` flags
- [ ] add regex? 
