// post.rs

use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::PathBuf;

fn read_file_string(filepath: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let markdown_str = fs::read_to_string(filepath)?;
    Ok(markdown_str)
}

pub fn get_html(postid: u32) -> Result<String, Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(r"static/posts/");
    path.push(postid.to_string());
    path.set_extension("md");
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let markdown_input = read_file_string(&path)?;
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(html_output)
}
