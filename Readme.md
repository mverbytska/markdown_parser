## Markdown Parser in Rust
### Simple parser for Markdown language written in Rust as a part of course project at NaUKMA

### Installation

### Option 1: run the following Cargo command in your project directory 
```
cargo add markdown_parser
```

### Option 2: add the following line to the *[dependencies]* section in Cargo.toml file

```
markdown_parser="0.1.0" 
```

### Technical overview:

#### 1. Takes string in markdown style as an input
```
fn main() {
    println!("{}", markdown_parser::to_html("# Hello! My name is *Mariia*."));
}
```

#### 2. The output will be a string with HTML tags to represent the result

```
<h1>Hello! My name is <em>Mariia</em>.</h1>
```
#### *KMA, Autumn, 2023*  