use std::fmt;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

#[derive(Debug)]
struct LinkEntry {
    url: String,
    title: String,
    tags: Option<Vec<String>>
}

impl fmt::Display for LinkEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "url: {}\ntitle: {}{}", self.url, self.title, match self.tags {
            None => "",
            Some(tags) => String::from("\ntags: ") + tags.join(",")
        })
    }
}

#[derive[Debug]]
struct ReadLaterList {
    links: HashMap<String, LinkEntry>
}

impl ReadLaterList {
    pub fn new() -> ReadLaterList {
        ReadLaterList {
            links: HashMap::new()
        }
    }

    // TODO
    pub fn parse(text: String) -> ReadLaterList {
        
    }

    // TODO how to handle collisions?
    pub fn addLink(&mut self, link: LinkEntry) -> &mut ReadLaterList {
        self.links.insert(link.url, link);
        self
    }

    pub fn getLink(&self, url: String) -> Option<LinkEntry> {
        self.links.get(url)

    pub fn updateLink(&mut self, url: String, newLink: LinkEntry) -> &mut ReadLaterList {
        self.links.insert(link.url, link);
        self
    }

    pub fn deleteLink(&mut self, url: String) -> &mut ReadLaterList {
        self.links.remove(url);
        self
    }

}

impl fmt::Display for ReadLaterList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.links.values().join("\n---\n"))
    }
}
