/// Tests for markdown parser

#[path = "../src/lib.rs"]
mod lib;
use lib::to_html;


#[cfg(test)]


/// Heading test testing
    #[test]
    fn test_heading() -> anyhow::Result< () > {
        assert_eq!(to_html("# This is a heading\n").unwrap(), "<h1>This is a heading</h1>");
        assert_eq!(to_html("## This is another heading\n").unwrap(), "<h1>This is another heading</h1>");

        Ok(())
    }

/// Bold text testing
    #[test]
    fn test_bold() -> anyhow::Result< () > {
        assert_eq!(to_html("**This is bold text**").unwrap(), "<strong>This is bold text</strong>");

        Ok(())
    }

/// Plain text testing
    #[test]
    fn test_text() -> anyhow::Result< () > {
        assert_eq!(to_html("This is regular text").unwrap(), "<p>This is regular text</p>");

        Ok(())
    }


/// Link text testing
    #[test]
    fn test_link() -> anyhow::Result< () >  {
        assert_eq!(to_html("[Link](https://example.com)").unwrap(), "<a href=\"https://example.com\">Link</a>");

        Ok(())
    }

