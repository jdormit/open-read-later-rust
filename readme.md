# open-read-later-rust
> A Rust implementation and CLI for the [Open Read-Later](https://github.com/jdormit/open-read-later) specification

## CLI Installation and Usage
The `readlater` binary can be installed with [Cargo](http://doc.crates.io/):

```bash
$ cargo install open_read_later
```

Usage:

```
readlater 1.0.0
Jeremy Dormitzer <jeremy.dormitzer@gmail.com>
Stores, queries, and manipulates read-later lists in the Open Read-Later specification format

USAGE:
    readlater [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    specifies the location of the list file [default: ~/.read_later_list]

SUBCOMMANDS:
    delete    deletes a link entry
    help      Prints this message or the help of the given subcommand(s)
    list      lists link entries
    save      saves or updates a link entry [aliases: update, add]
    search    searches link entries by keyword
    show      shows a link entry
    tag       adds or removes tags
```

## API Documentation
`open-read-later-rust` provides an API to manipulate and query read-later lists programmatically. See [the API documentations](https://docs.rs/open_read_later/1.0.0/open_read_later) for details.
