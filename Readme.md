## Markdown Parser in Rust
### Simple parser for Markdown language written in Rust as a part of course project at NaUKMA

### Link to this parser on crates.io: 

### Project Structure file: 
#### ./project_structure.txt

### Installation

### Run the following Cargo command in your project directory 
```
git clone https://github.com/mverbytska/markdown_parser.git
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

### In order to test the project, you can execute the following:
```
cargo test
```
#### *KMA, Autumn, 2023*  