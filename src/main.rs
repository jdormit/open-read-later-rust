extern crate open_read_later;
extern crate clap;

use std::env;
use std::io::Read;
use std::fs::OpenOptions;
use open_read_later::read_later_list::ReadLaterList;
use clap::{Arg, App, SubCommand};

fn main() {
    // TODO refactor to prevent excessive nesting
    match env::home_dir() {
        None => {
            println!("Unable to determine home directory. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new");
        },
        Some(home_dir) => {
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
            match list_file_result {
                Err(err) => println!("Error opening list file: {}. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new", err),
                Ok(mut list_file) => {
                    let mut list_text = String::new();
                    match list_file.read_to_string(&mut list_text) {
                        Err(err) => println!("Error reading list file: {}. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new", err),
                        Ok(_) => {
                            match ReadLaterList::parse(&list_text) {
                                Err(err) => println!("Error opening list file: {}. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new", err),
                                Ok(read_later_list) => {
                                    
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
