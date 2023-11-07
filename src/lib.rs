
use pest_derive::Parser;
pub use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct MarkdownParser;

