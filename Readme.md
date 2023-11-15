## Markdown Parser in Rust
### Simple parser for Markdown language written in Rust as a part of course project at NaUKMA

### Link to this parser on crates.io: 

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

#### 1. Takes a text file with a single string in markdown style as an input
```
cargo run to_html </path/to/file.md>

```

#### 2. The output will be a string with HTML tags to represent the result

```
<h1>Hello! My name is Mariia.</h1>
```

### Check documentation 

#### In order to see the documentation, you can use the following commands in your command line:
```
cargo doc

cargo doc --open
```
#### *KMA, Autumn, 2023*  