use pest_derive::Parser;
use pest::Parser as _;
use anyhow::{anyhow, Result};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct MarkdownParser;

/// Converts markdown-styled string into html-formatted one
/// # Arguments:
/// * argument: string in markdown style
/// # Returns: 
/// * formatted string with html tags
pub fn to_html(input: &str) -> Result<String> {
    let pairs = MarkdownParser::parse(Rule::content, input)?;

    let mut html_output = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::heading => {
                let heading_text = pair.as_str();
                html_output.push_str(&format!("<h1>{}</h1>", heading_text.trim_start_matches('#').trim()));
            }
            Rule::bold => {
                let bold_text = pair.as_str();
                html_output.push_str(&format!("<strong>{}</strong>", bold_text.trim_start_matches("**").trim_end_matches("**").trim()));
            }
            Rule::text => {
                let text_content = pair.as_str();
                html_output.push_str(&format!("<p>{}</p>", text_content));
            }
            Rule::list => {
                let list_item = pair.as_str();
                html_output.push_str(&format!("<li>{}</li>", list_item.trim_start_matches('-').trim()));
            }                          
            Rule::link => {
                let mut inner_pairs = pair.into_inner();
                let link_text = inner_pairs.next().ok_or_else(|| anyhow!("No inner pair found for link text"))?.as_str();
                let link_url = inner_pairs.next().ok_or_else(|| anyhow!("No inner pair found for link URL"))?.as_str();
                html_output.push_str(&format!("<a href=\"{}\">{}</a>", link_url, link_text));
            }
            _ => {
                return Err(anyhow!("Unexpected rule: {:?}", pair.as_rule()));
            }
        }
    }

    Ok(html_output)
}

