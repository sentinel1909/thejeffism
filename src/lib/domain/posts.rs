// posts.rs

use pulldown_cmark::{html, Options, Parser};

pub fn get_markdown() -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let markdown_str = r#"
## First Post
### 2022-10-09
Every site has to begin somewhere. This content is written in markdown and rendered to the page thanks to pulldown-cmark!
"#;

    let parser = Parser::new_ext(markdown_str, options);

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}
