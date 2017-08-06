extern crate open_read_later;
extern crate clap;

use std::env;
use std::io::Read;
use std::fs::OpenOptions;
use open_read_later::read_later_list::ReadLaterList;
use clap::{Arg, App, SubCommand};

fn main() {
    match run() {
        Err(err) => println!("{}", err),
        Ok(_) => return
    }
}

fn run() -> Result<i32, String> {
    // TODO reformat to use try! or '?'
    let home_dir = match env::home_dir() {
        None =>  return Err("Unable to determine home directory. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new".to_string()),
        Some(home_dir) => home_dir
    };
    let mut default_list_file = home_dir;
    default_list_file.push(".read_later_list");

    let matches = App::new("readlater")
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
                .get_matches();

    let list_file_path = matches.value_of("file").unwrap();
    let list_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(list_file_path);
    let mut list_file = match list_file_result {
        Err(err) => return Err(format!("Error opening list file: {}. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new", err)),
        Ok(mut list_file) => list_file
    };
    let mut list_text = String::new();
    list_file.read_to_string(&mut list_text);

    let read_later_list = match ReadLaterList::parse(&list_text) {
        Err(err) => return Err(format!("Error opening list file: {}. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new", err)),
        Ok(read_later_list) => read_later_list
    };

    Ok(1)
}
