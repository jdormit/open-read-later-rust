extern crate open_read_later;
extern crate clap;

mod util;

use std::env;
use std::error::Error;
use std::path::PathBuf;
use open_read_later::read_later_list::{ReadLaterList, LinkEntry};
use util::{prompt, read_from_file, overwrite_file};
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
    let list_text = read_from_file(list_file_path).unwrap_or(String::from(""));

    let mut read_later_list = ReadLaterList::parse(&list_text)?;

    match args.subcommand() {
        ("list", Some(_)) => list(&read_later_list),
        ("save", Some(save_args)) => save(&mut read_later_list, save_args)?,
        ("show", Some(show_args)) => show(&read_later_list, show_args),
        ("delete", Some(delete_args)) => delete(&mut read_later_list, delete_args),
        ("tags", Some(tags_args)) => tags(&mut read_later_list, tags_args)?,
        _ => println!("{}", args.usage()),
    };

    overwrite_file(list_file_path, &read_later_list.to_string())?;

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
                    .visible_aliases(&["update", "add"])
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
        .subcommand(SubCommand::with_name("list")
                    .about("lists link entries"))
        .subcommand(SubCommand::with_name("search")
                    .about("searches link entries by keyword")
                    .arg(Arg::with_name("keyword")
                         .takes_value(true)
                         .value_name("KEYWORD")
                         .required(true)))
        .subcommand(SubCommand::with_name("show")
                    .about("shows a link entry")
                    .arg(Arg::with_name("url")
                         .help("the URL of the link to show")
                         .takes_value(true)
                         .value_name("URL")
                         .required(true)))
        .subcommand(SubCommand::with_name("delete")
                    .about("deletes a link entry")
                    .arg(Arg::with_name("url")
                         .help("the URL of the link to delete")
                         .takes_value(true)
                         .value_name("URL")
                         .required(true)))
        .subcommand(SubCommand::with_name("tag")
                    .about("adds or removes tags")
                    .subcommand(SubCommand::with_name("add")
                                .about("adds tags to a link")
                                .arg(Arg::with_name("url")
                                     .help("the URL of the link")
                                     .takes_value(true)
                                     .value_name("URL")
                                     .required(true))
                                .arg(Arg::with_name("tags")
                                     .help("tags to add")
                                     .takes_value(true)
                                     .value_name("TAG")
                                     .multiple(true)
                                     .min_values(1)))
                    .subcommand(SubCommand::with_name("remove")
                                .about("removes tags from a link")
                                .arg(Arg::with_name("url")
                                     .help("the URL of the link")
                                     .takes_value(true)
                                     .value_name("URL")
                                     .required(true))
                                .arg(Arg::with_name("tags")
                                     .help("tags to add")
                                     .takes_value(true)
                                     .value_name("TAG")
                                     .multiple(true)
                                     .min_values(1))))
        .get_matches()
}

fn list(read_later_list: &ReadLaterList) {
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
    let tags: Vec<String> = match save_args.values_of("tags") {
        None => {
            let mut buffer = String::new();
            prompt("[Optional] Enter comma-separated tags: ", &mut buffer)?;
            buffer.split(",").map(|tag| tag.trim()).map(String::from).collect()
        }
        Some(tags) => tags.map(String::from).collect()
    };
    let link_entry = LinkEntry::builder()
        .set_url(url)
        .set_title(&title)
        .add_tags(&mut tags.iter().map(|tag| tag.as_ref()).collect())
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

fn delete(read_later_list: &mut ReadLaterList, args: &ArgMatches) {
    let url = args.value_of("url").unwrap();
    read_later_list.delete_link(url);
}

fn tags(read_later_list: &mut ReadLaterList, args: &ArgMatches) -> Result<(), Box<Error>> {
    match args.subcommand() {
        ("add", Some(add_args)) => {
            let url = add_args.value_of("url").unwrap();
            let tags = add_args.values_of("tags").unwrap();
            match read_later_list.get_link(url) {
                None => println!("Link {} not found", url),
                Some(_) => { read_later_list.add_tags(url, tags.map(String::from).collect())?; }
            }
        },
        ("remove", Some(remove_args)) => {
            let url = remove_args.value_of("url").unwrap();
            let tags = remove_args.values_of("tags").unwrap();
            match read_later_list.get_link(url) {
                None => println!("Link {} not found", url),
                Some(_) => { read_later_list.remove_tags(url, tags.map(String::from).collect())?; }
            }
        },
        _ => println!("{}", args.usage())
    };
    Ok(())
}
