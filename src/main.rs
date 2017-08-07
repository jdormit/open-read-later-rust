extern crate open_read_later;
extern crate clap;

mod util;

use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::OpenOptions;
use open_read_later::read_later_list::{ReadLaterList, LinkEntry};
use util::prompt;
use clap::{Arg, App, SubCommand, ArgMatches};

fn main() {
    match run() {
        Err(err) => {
            println!(
                "Encountered error: {}. Please file an issue at https://github.com/jdormit/open-read-later-rust/issues/new",
                err)
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

    let mut read_later_list = ReadLaterList::parse(&list_text)?;

    match args.subcommand() {
        ("list", Some(list_args)) => list(&read_later_list, list_args),
        ("save", Some(save_args)) => save(&mut read_later_list, save_args)?,
        ("show", Some(show_args)) => show(&read_later_list, show_args),
        _ => println!("{}", args.usage()),
    };

    list_file.write(&read_later_list.to_string().into_bytes())?;

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
                    .about("saves a new link entry")
                    .arg(Arg::with_name("url")
                         .help("the URL of the link to save")
                         .takes_value(true)
                         .value_name("URL")
                         .required(true))
                    .arg(Arg::with_name("title")
                         .help("the title of the link to save")
                         .long("title")
                         .takes_value(true)
                         .value_name("TITLE"))
                    .arg(Arg::with_name("tags")
                         .help("tags to apply to the link")
                         .long("tags")
                         .takes_value(true)
                         .value_name("TAGS")
                         .multiple(true)))
        .subcommand(SubCommand::with_name("edit")
                    .about("edits a link entry"))
        .subcommand(SubCommand::with_name("list")
                    .about("lists link entries"))
        .subcommand(SubCommand::with_name("search")
                    .about("searches link entries by keyword"))
        .subcommand(SubCommand::with_name("show")
                    .about("shows a link entry")
                    .arg(Arg::with_name("url")
                         .help("the URL of the link to show")
                         .takes_value(true)
                         .value_name("URL")
                         .required(true)))
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

fn save(read_later_list: &mut ReadLaterList, save_args: &ArgMatches) ->  Result<(), Box<Error>> {
    let url = save_args.value_of("url").unwrap();
    println!("Saving link {}", url);
    let title = match save_args.value_of("title") {
        None => {
            let mut buffer = String::new();
            prompt("Enter link title: ", &mut buffer)?;
            buffer
        },
        Some(title) => String::from(title)
    };
    let mut tags = match save_args.values_of("tags") {
        // TODO add interactive dialog to add tags
        None => Vec::new(),
        Some(tags) => tags.collect()
    };
    let link_entry = LinkEntry::builder()
        .set_url(url)
        .set_title(&title)
        .add_tags(&mut tags)
        .build()?;
    read_later_list.add_link(link_entry);
    Ok(())
}

fn show(read_later_list: &ReadLaterList, args: &ArgMatches) {
    let url = args.value_of("url").unwrap();
    match read_later_list.get_link(url) {
        None => println!("Link {} not found", url),
        Some(link_entry) => println!("{}", link_entry)
    }
}
