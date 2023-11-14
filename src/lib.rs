use pest_derive::Parser;
use pest::Parser as _; 
use anyhow::{anyhow, Result}; 

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct MarkdownParser;

pub fn to_markdown(input: &str) -> Result<()> {
    let pairs = MarkdownParser::parse(Rule::markdown, input)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::heading => {
                println!("Heading: {}", pair.as_str());
            }
            Rule::bold => {
                println!("Bold: {}", pair.into_inner().next().unwrap().as_str());
            }
            Rule::text => {
                println!("Text: {}", pair.as_str());
            }
            Rule::list => {
                println!("List: {}", pair.into_inner().next().unwrap().as_str());
            }
            Rule::link => {
                let mut inner_pairs = pair.into_inner();
                let link_text = inner_pairs.next().unwrap().as_str();
                let link_url = inner_pairs.next().unwrap().as_str();
                println!("Link: [{}]({})", link_text, link_url);
            }
            _ => {
                return Err(anyhow!("Mismatched input"));
            }
        }
    }

    Ok(())
}