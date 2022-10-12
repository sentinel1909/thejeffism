// posts.rs

use pulldown_cmark::{html, Parser};

pub fn get_markdown() -> String {
    let markdown_str = r#"
## First Post
### 2022-10-09
Every site has to begin somewhere. This content is written in markdown and rendered to the page thanks to pulldown-cmark!
"#;

    let parser = Parser::new(markdown_str);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}