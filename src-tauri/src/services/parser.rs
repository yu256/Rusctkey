use super::service::add_emojis;
use anyhow::Result;
use html_escape::encode_text;
use regex::{Captures, Regex};
use std::collections::HashMap;

pub(crate) fn parse_text(
    text: &str,
    emojis: &HashMap<String, String>,
    is_local: bool,
) -> Result<String> {
    let mut parsed_text = String::new();

    let encoded = encode_text(text);
    let lines: Vec<&str> = encoded.lines().collect();

    for line in lines {
        let mut line = parse_url(line);

        parse_search_links(&mut line);

        // let line = parse_twemoji(&line);

        let line = parse_customemojis(&line, &emojis, is_local)?;

        // let line = parse_mfm(&line);

        // let line = parse_code_block(&line);

        parsed_text.push_str(&line);
        parsed_text.push_str("<br>");
    }

    Ok(parsed_text)
}

fn parse_url(line: &str) -> String {
    let regex = Regex::new(r"https?://\S+").unwrap();
    let replaced_line = regex.replace_all(&line, |caps: &Captures| {
        let url = &caps[0];
        format!("<a href=\"{}\">{}</a>", url, url)
    });
    replaced_line.to_string()
}

fn parse_search_links(line: &mut String) {
    let regex = Regex::new(r"(.*)\s+(?:search|検索|\[search\]|\[検索\])").unwrap();
    *line = regex.replace_all(&line, |caps: &Captures| {
        format!(
            "<input type=\"search\" value=\"{}\"><button onclick=\"search(this.previousElementSibling.value)\">検索</button>",
            &caps[1]
        )
    }).to_string();
}

pub(crate) fn parse_customemojis(
    line: &str,
    emojis: &HashMap<String, String>,
    is_local: bool,
) -> Result<String> {
    let regex = Regex::new(r":(\w+):")?;
    let replaced_line = regex.replace_all(line, |caps: &Captures| {
        customemojis_to_html(&caps[1], emojis, is_local).unwrap()
    });
    Ok(replaced_line.to_string())
}

fn customemojis_to_html(
    name: &str,
    emojis: &HashMap<String, String>,
    is_local: bool,
) -> Result<String> {
    let url = if is_local {
        add_emojis(name)?
    } else {
        if let Some(url) = emojis.get(name) {
            url.to_string()
        } else {
            String::new()
        }
    };

    let style = "display: inline; width: auto; height: 2em; max-width: 100%;";
    if url.is_empty() {
        Ok(format!(":{}:", name))
    } else {
        Ok(format!(
            "<img src=\"{}\" alt=\"{}\" style=\"{}\">",
            url, name, style
        ))
    }
}
