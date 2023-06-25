use std::collections::HashMap;

use ammonia::Builder;
use regex::{Captures, Regex};

use super::service::add_emojis;

pub(crate) fn parse_text(text: &str, emojis: &HashMap<String, String>) -> String {
    let html = sanitize_html(text);
    let mut parsed_text = String::new();

    let lines: Vec<&str> = html.lines().collect();

    for line in lines {
        let line = parse_url(&line);

        let line = parse_search_links(&line);

        // let line = parse_twemoji(&line);

        let line = parse_customemojis(&line, &emojis);

        // let line = parse_mfm(&line);

        // let line = parse_code_block(&line);

        parsed_text.push_str(&line);
        parsed_text.push_str("<br>");
    }

    parsed_text
}

pub(crate) fn parse_username(text: &str, emojis: &HashMap<String, String>) -> String {
    let html = sanitize_html(text);
    parse_customemojis(&html, &emojis)
}

fn sanitize_html(text: &str) -> String {
    let builder = Builder::default();

    builder.clean(text).to_string()
}

fn parse_url(line: &str) -> String {
    let url_regex = Regex::new(r"\bhttps://\S+").unwrap();
    let replaced_line = url_regex.replace_all(&line, |caps: &Captures| {
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

fn parse_customemojis(line: &str, emojis: &HashMap<String, String>) -> String {
    let regex = Regex::new(r":(\w+):").unwrap();
    let replaced_line = regex.replace_all(line, |caps: &Captures| {
        let emoji_code = caps.get(1).unwrap().as_str();
        customemojis_to_html(emoji_code, emojis)
    });
    replaced_line.to_string()
}

fn customemojis_to_html(name: &str, emojis: &HashMap<String, String>) -> String {
    let url = if let Some(url) = emojis.get(name) {
        url.to_string()
    } else {
        add_emojis(name)
    };

    let style = "display: inline; width: auto; height: 2em; max-width: 100%;";
    if url.is_empty() {
        format!(":{}:", name)
    } else {
        format!("<img src=\"{}\" alt=\"{}\" style=\"{}\">", url, name, style)
    }
}
