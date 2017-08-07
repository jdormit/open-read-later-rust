extern crate open_read_later;
extern crate clap;

use std::env;
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;
use std::fs::OpenOptions;
use open_read_later::read_later_list::ReadLaterList;
use clap::{Arg, App, SubCommand, ArgMatches};

fn main() {
    match run() {
        Err(err) => {
            println!(
                "Encountered error: {}. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new",
                err
            )
        }
        Ok(_) => return,
    }
}

fn run() -> Result<i32, Box<Error>> {
    let home_dir = env::home_dir().ok_or("Cannot find home directory")?;
    let mut default_list_file = home_dir;
    default_list_file.push(".read_later_list");

    let args = parse_args(&default_list_file);

    let list_file_path = args.value_of("read_later_file").unwrap();
    let mut list_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(list_file_path)?;

    let mut list_text = String::new();
    list_file.read_to_string(&mut list_text)?;

    let read_later_list = ReadLaterList::parse(&list_text)?;

    match args.subcommand() {
        ("list", Some(list_args)) => { list(&read_later_list, list_args); },
        _ => {
            args.usage();
        }
    }

    Ok(0)
}

fn parse_args<'a>(default_list_file: &'a PathBuf) -> ArgMatches<'a> {
    App::new("readlater")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jeremy Dormitzer <jeremy.dormitzer@gmail.com>")
        .about("Stores, queries, and manipulates read-later lists in the Open Read-Later specification format")
        .arg(Arg::with_name("read_later_file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("specifies the location of the list file")
             .takes_value(true)
             .default_value(default_list_file.to_str().unwrap()))
        .subcommand(SubCommand::with_name("save")
                    .about("saves a new link entry"))
        .subcommand(SubCommand::with_name("edit")
                    .about("edits a link entry"))
        .subcommand(SubCommand::with_name("list")
                    .about("lists link entries"))
        .subcommand(SubCommand::with_name("search")
                    .about("searches link entries by keyword"))
        .subcommand(SubCommand::with_name("show")
                    .about("shows a link entry"))
        .subcommand(SubCommand::with_name("delete")
                    .about("deletes a link entry"))
        .get_matches()
}

fn list(read_later_list: &ReadLaterList, list_args: &ArgMatches) {
    match read_later_list.len() {
        0 => {
            println!("Read-later list empty");
        }
        _ => {
            println!("{}", read_later_list);
        }
    }
}
