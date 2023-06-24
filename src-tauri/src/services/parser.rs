use ammonia::Builder;
use regex::{Captures, Regex};

pub(crate) fn parse_text(text: &str) -> String {
    let html = sanitize_html(text);
    let mut parsed_text = String::new();

    let lines: Vec<&str> = html.lines().collect();

    for line in lines {
        let line = parse_url(&line);

        let line = parse_search_links(&line);

        // let line = parse_twemoji(&line);

        // let line = parse_customemojis(&line);

        // let line = parse_mfm(&line);

        // let line = parse_code_block(&line);

        parsed_text.push_str(&line);
        parsed_text.push_str("<br>");
    }

    parsed_text
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
    let regex = Regex::new(r"(?i)(.*)\s+(?:\[|\bsearch\b|\b検索\b\]?)").unwrap();
    let replaced_line = regex.replace_all(line, |caps: &Captures| {
        let search_query = caps.get(1).unwrap().as_str();
        format!(
            "<input type=\"search\" value=\"{}\"><button onclick=\"search(this.previousElementSibling.value)\">検索</button>",
            search_query
        )
    });

    replaced_line.to_string()
}
