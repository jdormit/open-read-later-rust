extern crate open_read_later;

use open_read_later::read_later_list::{LinkEntry, ReadLaterList};

#[test]
fn it_parses_read_later_list() {
    let read_later_text = "\
url: https://example.com
title: Example
tags: tag1, tag2
---
url: https://jeremydormitzer.com
title: Jeremy";

    let read_later_list_parsed = ReadLaterList::parse(read_later_text);
    let read_later_list_constructed = ReadLaterList::new()
        .add_link(
            LinkEntry::builder()
                .set_url("https://example.com")
                .set_title("Example")
                .add_tags(&mut vec!["tag1", "tag2"])
                .build()
                .unwrap(),
        )
        .add_link(
            LinkEntry::builder()
                .set_url("https://jeremydormitzer.com")
                .set_title("Jeremy")
                .build()
                .unwrap(),
        );

    assert!(read_later_list_parsed.is_ok());
    assert_eq!(read_later_list_parsed.unwrap(), read_later_list_constructed);
}

#[test]
fn it_outputs_read_later_string() {
    let read_later_text = "\
url: https://example.com
title: Example
tags: tag1, tag2
---
url: https://jeremydormitzer.com
title: Jeremy";

    let read_later_list_parsed = ReadLaterList::parse(read_later_text);
    let read_later_list_constructed = ReadLaterList::new()
        .add_link(
            LinkEntry::builder()
                .set_url("https://example.com")
                .set_title("Example")
                .add_tags(&mut vec!["tag1", "tag2"])
                .build()
                .unwrap(),
        )
        .add_link(
            LinkEntry::builder()
                .set_url("https://jeremydormitzer.com")
                .set_title("Jeremy")
                .build()
                .unwrap(),
        );

    assert_eq!(read_later_list_parsed.unwrap().to_string(), read_later_text);
    assert_eq!(read_later_list_constructed.to_string(), read_later_text);
}

#[test]
fn it_adds_tags() {
    let read_later_text = "\
url: https://example.com
title: Example
tags: tag1, tag2
---
url: https://jeremydormitzer.com
title: Jeremy";

    let read_later_text_with_tags = "\
    url: https://example.com
title: Example
tags: tag1, tag2
---
url: https://jeremydormitzer.com
title: Jeremy
tags: The coolest, amazing, best";

    let mut read_later_list = ReadLaterList::parse(read_later_text).unwrap();
    read_later_list
        .add_tags(
            "https://jeremydormitzer.com",
            vec!["The coolest", "amazing", "best"]
                .iter()
                .map(|&tag| String::from(tag))
                .collect(),
        )
        .unwrap();

    let read_later_list_constructed = ReadLaterList::new()
        .add_link(
            LinkEntry::builder()
                .set_url("https://example.com")
                .set_title("Example")
                .add_tags(&mut vec!["tag1", "tag2"])
                .build()
                .unwrap(),
        )
        .add_link(
            LinkEntry::builder()
                .set_url("https://jeremydormitzer.com")
                .set_title("Jeremy")
                .add_tags(&mut vec!["The coolest", "amazing", "best"])
                .build()
                .unwrap(),
        );

    assert_eq!(read_later_list, read_later_list_constructed);
    assert_eq!(read_later_list.to_string(), read_later_text_with_tags);
}

#[test]
fn it_removes_tags() {
    let read_later_text = "\
url: https://example.com
title: Example
tags: tag1, tag2
---
url: https://jeremydormitzer.com
title: Jeremy";

    let read_later_text_without_tags = "\
url: https://example.com
title: Example
---
url: https://jeremydormitzer.com
title: Jeremy";

    let mut read_later_list = ReadLaterList::parse(read_later_text).unwrap();
    read_later_list
        .remove_tags(
            "https://example.com",
            vec!["tag1", "tag2"]
                .iter()
                .map(|&tag| String::from(tag))
                .collect(),
        )
        .unwrap();

    let read_later_list_constructed = ReadLaterList::new()
        .add_link(
            LinkEntry::builder()
                .set_url("https://example.com")
                .set_title("Example")
                .build()
                .unwrap(),
        )
        .add_link(
            LinkEntry::builder()
                .set_url("https://jeremydormitzer.com")
                .set_title("Jeremy")
                .build()
                .unwrap(),
        );

    assert_eq!(read_later_list, read_later_list_constructed);
    assert_eq!(read_later_list.to_string(), read_later_text_without_tags);
}
