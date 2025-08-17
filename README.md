A command-line tool to find a given input in a given file or directory.

Example usage: `grrs <input> [<location>]`

[reference](https://rust-cli.github.io/book/)


TODO:
- [x] fix verbosity (get rid of if else hell)
- [x] add color highlighting to matches in files
- [] add file name where the phrase was found
- ~~[] do this in C~~
- ~~[] move reusable code into function (probably the stuff with output flag)~~
- [] handle all unwraps :joy:
- [x] fix --output flag recursion bug (i just needed a newline lmfao)