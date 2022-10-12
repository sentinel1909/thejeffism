// posts.rs

use pulldown_cmark::{html, Parser};

pub fn get_markdown() -> String {
    let markdown_str = r#"
    # Test
    ## Markdown Content
    This is some test markdown content.
    "#;

    let parser = Parser::new(markdown_str);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}