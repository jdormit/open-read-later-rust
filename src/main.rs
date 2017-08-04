extern crate open_read_later;
extern crate clap;

use open_read_later::read_later_list::ReadLaterList;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("readlater")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Jeremy Dormitzer <jeremy.dormitzer@gmail.com>")
        .about("Stores, queries, and manipulates read-later lists in the Open Read-Later specification format")
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
}
