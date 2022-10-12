// posts.rs

use pulldown_cmark::{html, Parser};

pub fn get_markdown() -> String {
    let markdown_str = "
        ## Test
        ### Markdown
        
        This is some test in markdown.";

    let parser = Parser::new(markdown_str);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}