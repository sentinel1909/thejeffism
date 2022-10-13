// posts.rs

use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::Path;

fn read_file_string(filepath: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let markdown_str = fs::read_to_string(filepath)?;
    Ok(markdown_str)
}

pub fn get_html() -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new("static/posts/20221011.md");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let markdown_input = read_file_string(path)?;
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}
