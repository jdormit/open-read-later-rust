use std::fmt;
use std::io;
use std::result::Result;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct LinkEntry {
    url: String,
    title: String,
    tags: Option<Vec<String>>,
}

struct LinkEntryBuilder {
    url: Option<String>,
    title: Option<String>,
    tags: Vec<String>,
}

impl LinkEntryBuilder {
    fn new() -> LinkEntryBuilder {
        LinkEntryBuilder {
            url: None,
            title: None,
            tags: Vec::new(),
        }
    }

    fn setUrl(&mut self, url: &str) -> &mut LinkEntryBuilder {
        self.url = Some(String::from(url));
        self
    }

    fn setTitle(&mut self, title: &str) -> &mut LinkEntryBuilder {
        self.title = Some(String::from(title));
        self
    }

    fn addTag(&mut self, tag: &str) -> &mut LinkEntryBuilder {
        self.tags.push(String::from(tag));
        self
    }

    fn build(&self) -> Result<LinkEntry, &str> {
        match self.url {
            None => Err("URL not set"),
            Some(url) => match self.title {
                None => Err("title not set"),
                Some(title) => match self.tags.len() {
                    0 => Ok(LinkEntry {
                        url: url,
                        title: title,
                        tags: None,
                    }),
                    _ => Ok(LinkEntry {
                        url: url,
                        title: title,
                        tags: Some(self.tags),
                    }),
                }
            }
        }
    }
}

impl LinkEntry {
    fn parse(text: String) -> io::Result<LinkEntry> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"^(.+?):\s?(.+)$").unwrap();
        }
        re.captures_iter(&text)
            .fold(LinkEntryBuilder::new(), |builder, cap| match cap[1].trim() {
                "url" => builder.setUrl(cap[2].trim()),
                "title" => builder.setTitle(cap[2].trim()),
                "tags" => cap[2]
                    .trim()
                    .split(",")
                    .fold(builder, |builder, tag| builder.addTag(tag.trim())),
                _ => builder
            }).build()
    }
}

impl fmt::Display for LinkEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "url: {}\ntitle: {}{}", self.url, self.title, match self.tags {
            None => String::from(""),
            Some(tags) => String::from("\ntags: ") + &tags.join(",")
        })
    }
}

#[derive(Debug)]
struct ReadLaterList {
    links: HashMap<String, LinkEntry>,
}

impl ReadLaterList {
    pub fn new() -> ReadLaterList {
        ReadLaterList {
            links: HashMap::new()
        }
    }

    pub fn parse(text: String) -> io::Result<ReadLaterList> {
        text.split("\n---\n")
            .fold(Ok(ReadLaterList::new()),
                  |readLaterListResult, linkText| match readLaterListResult {
                      Error => Error,
                      Ok(readLaterList) => match LinkEntry::parse(String::from(linkText)) {
                          Error => Error,
                          Ok(linkEntry) => Ok(readLaterList.addLink(linkEntry)),
                      }
                  })
    }

    // TODO how to handle collisions?
    pub fn addLink(&mut self, link: LinkEntry) -> &mut ReadLaterList {
        self.links.insert(link.url, link);
        self
    }

    pub fn getLink(&self, url: &str) -> Option<&LinkEntry> {
        self.links.get(url)
    }

    pub fn updateLink(&mut self, url: &str, newLink: LinkEntry) -> &mut ReadLaterList {
        self.links.insert(newLink.url, newLink);
        self
    }

    pub fn deleteLink(&mut self, url: &str) -> &mut ReadLaterList {
        self.links.remove(url);
        self
    }
}

impl fmt::Display for ReadLaterList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.links.values()
               .map(|link| link.to_string())
               .collect::<Vec<String>>()
               .join("\n---\n"))
    }
}
