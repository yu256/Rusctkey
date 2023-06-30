use std::collections::HashMap;

use html_escape::encode_text;
use regex::{Captures, Regex};

use super::service::add_emojis;

pub(crate) fn parse_text(text: &str, emojis: &HashMap<String, String>, is_local: bool) -> String {
    let mut parsed_text = String::new();

    let encoded = encode_text(text);
    let lines: Vec<&str> = encoded.lines().collect();

    for line in lines {
        let line = parse_url(&line);

        let line = parse_search_links(&line);

        // let line = parse_twemoji(&line);

        let line = parse_customemojis(&line, &emojis, is_local);

        // let line = parse_mfm(&line);

        // let line = parse_code_block(&line);

        parsed_text.push_str(&line);
        parsed_text.push_str("<br>");
    }

    parsed_text
}

fn parse_url(line: &str) -> String {
    let regex = Regex::new(r"https?://\S+").unwrap();
    let replaced_line = regex.replace_all(&line, |caps: &Captures| {
        let url = caps.get(0).unwrap().as_str();
        format!("<a href=\"{}\">{}</a>", url, url)
    });
    replaced_line.to_string()
}

fn parse_search_links(line: &str) -> String {
    let regex = Regex::new(r"(.*)\s+(?:search|検索|\[search\]|\[検索\])").unwrap();
    let replaced_line = regex.replace_all(line, |caps: &Captures| {
        let search_query = caps.get(1).unwrap().as_str();
        format!(
            "<input type=\"search\" value=\"{}\"><button onclick=\"search(this.previousElementSibling.value)\">検索</button>",
            search_query
        )
    });

    replaced_line.to_string()
}

pub(crate) fn parse_customemojis(
    line: &str,
    emojis: &HashMap<String, String>,
    is_local: bool,
) -> String {
    let regex = Regex::new(r":(\w+):").unwrap();
    let replaced_line = regex.replace_all(line, |caps: &Captures| {
        let emoji_code = caps.get(1).unwrap().as_str();
        customemojis_to_html(emoji_code, emojis, is_local)
    });
    replaced_line.to_string()
}

fn customemojis_to_html(name: &str, emojis: &HashMap<String, String>, is_local: bool) -> String {
    let url = if is_local {
        add_emojis(name)
    } else {
        if let Some(url) = emojis.get(name) {
            url.to_string()
        } else {
            String::new()
        }
    };

    let style = "display: inline; width: auto; height: 2em; max-width: 100%;";
    if url.is_empty() {
        format!(":{}:", name)
    } else {
        format!("<img src=\"{}\" alt=\"{}\" style=\"{}\">", url, name, style)
    }
}
