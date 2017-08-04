use std::fmt;
use std::result::Result;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct LinkEntry {
    url: String,
    title: String,
    tags: Vec<String>,
}

pub struct LinkEntryBuilder {
    url: Option<String>,
    title: Option<String>,
    tags: Vec<String>,
}

impl LinkEntryBuilder {
    pub fn new() -> LinkEntryBuilder {
        LinkEntryBuilder {
            url: None,
            title: None,
            tags: Vec::new(),
        }
    }

    pub fn set_url(mut self, url: &str) -> LinkEntryBuilder {
        self.url = Some(String::from(url));
        self
    }

    pub fn set_title(mut self, title: &str) -> LinkEntryBuilder {
        self.title = Some(String::from(title));
        self
    }

    pub fn add_tag(mut self, tag: &str) -> LinkEntryBuilder {
        self.tags.push(String::from(tag));
        self
    }

    pub fn add_tags(mut self, tags: &mut Vec<&str>) -> LinkEntryBuilder {
        self.tags.append(&mut tags.iter().map(|&s| String::from(s)).collect::<Vec<String>>());
        self
    }

    pub fn build(self) -> Result<LinkEntry, String> {
        match self.url {
            None => Err(String::from("URL not set")),
            Some(url) => match self.title {
                None => Err(String::from("title not set")),
                Some(title) => match self.tags.len() {
                    0 => Ok(LinkEntry {
                        url: url,
                        title: title,
                        tags: Vec::new(),
                    }),
                    _ => Ok(LinkEntry {
                        url: url,
                        title: title,
                        tags: self.tags,
                    }),
                }
            }
        }
    }
}

impl LinkEntry {
    pub fn builder() -> LinkEntryBuilder {
        LinkEntryBuilder::new()
    }

    fn parse(text: &str) -> Result<LinkEntry, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.+?)\s*:\s*(.+)$").unwrap();
        }
        text.lines()
            .fold(LinkEntryBuilder::new(), |builder, line| match RE.captures(&line) {
                None => builder,
                Some(cap) => match cap[1].trim() {
                    "url" => builder.set_url(cap[2].trim()),
                    "title" => builder.set_title(cap[2].trim()),
                    "tags" => builder.add_tags(&mut cap[2]
                                               .trim()
                                               .split(",")
                                               .map(|s| s.trim())
                                               .collect::<Vec<&str>>()),
                    _ => builder
                }
            }).build()
    }
}

impl fmt::Display for LinkEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "url: {}\ntitle: {}{}", self.url, self.title, match self.tags.len() {
            0 => String::from(""),
            _ => String::from("\ntags: ") + &self.tags.join(", ")
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadLaterList {
    links: HashMap<String, LinkEntry>,
}

impl ReadLaterList {
    pub fn new() -> ReadLaterList {
        ReadLaterList {
            links: HashMap::new()
        }
    }

    pub fn parse(text: &str) -> Result<ReadLaterList, String> {
        text.split("\n---\n")
            .fold(Ok(ReadLaterList::new()),
                  |read_later_list_result, link_text| match read_later_list_result {
                      Err(msg) => Err(msg),
                      Ok(read_later_list) => match LinkEntry::parse(link_text) {
                          Err(msg) => Err(msg),
                          Ok(link_entry) => Ok(read_later_list.add_link(link_entry)),
                      }
                  })
    }

    pub fn add_link(mut self, link: LinkEntry) -> ReadLaterList {
        self.links.insert(link.url.clone(), link);
        self
    }

    pub fn get_link(&self, url: &str) -> Option<&LinkEntry> {
        self.links.get(url)
    }

    pub fn update_link(mut self, new_link: LinkEntry) -> ReadLaterList {
        self.links.insert(new_link.url.clone(), new_link);
        self
    }

    pub fn delete_link(mut self, url: &str) -> ReadLaterList {
        self.links.remove(url);
        self
    }
}

impl fmt::Display for ReadLaterList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vals = self.links.values()
            .map(|link| link.to_string())
            .collect::<Vec<String>>();
        vals.sort();
        write!(f, "{}", vals.join("\n---\n"))
    }
}
