use pest_derive::Parser;
use pest::Parser;
use anyhow::{anyhow, Result};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;
  pub fn main() -> anyhow::Result< () >{

     Ok( () )
  }

#[test]
pub fn basic_test() -> anyhow::Result< () >{
  let got = Grammar::parse(Rule::field, "121.87")?.next().ok_or_else(|| anyhow!("no pair"))?;
  dbg!(got);

  Ok( () )
}