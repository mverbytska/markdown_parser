/// Tests for markdown parser

#[cfg(test)]
mod tests {
    use pest_derive::Parser;
    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "../src/grammar.pest"] 
    struct MarkdownParser;

    #[test]
    fn test_heading() {
        assert!(MarkdownParser::parse(Rule::heading, "# This is a heading\n").is_ok());
        assert!(MarkdownParser::parse(Rule::heading, "## This is another heading\n").is_ok());
    }

    #[test]
    fn test_bold() {
        assert!(MarkdownParser::parse(Rule::bold, "**This is bold text**").is_ok());
    }
}

