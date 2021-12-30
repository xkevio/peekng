# peekng
Small Rust tool to add chunks with messages to a PNG image file based on the ["PNGme" tutorial series](https://picklenerd.github.io/pngme_book/introduction.html).

## Usage
Compile with `cargo build --release`.

`$ ./peekng`

```
peekng 0.1.0
xkevio

USAGE:
    peekng <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    decode    Decodes a message (prints it) if given a chunk type name
    encode    Encodes a secret message into the given image
    help      Print this message or the help of the given subcommand(s)
    print     Prints all chunks in the PNG
    remove    Removes chunk from PNG if chunk type exists
```