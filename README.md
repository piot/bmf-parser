# bmf-parser 🖋️

`bmf-parser` is a Rust crate for reading and parsing [BMFont](https://www.angelcode.com/products/bmfont/doc/file_format.html#bin) binary files, a popular font format for bitmap fonts often used in games and graphical applications. With `bmf-parser`, you can easily load BMFont data into a `BMFont` struct.

## ✨ Features

- **Simple API**: Load BMFont binary data directly into a `BMFont` structure using a single method.
- **Comprehensive Font Data**: Access font metadata, character properties, page details, and kerning pairs.
- **Efficient Parsing**: Designed to be fast and efficient, perfect for games and applications needing bitmap font support.

## 📦 Installation

To start using `bmf-parser`, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
bmf-parser = "0.0.1"
```

## Usage

```rust
use bmf_parser::BMFont;
use std::{fs, io};

fn main() -> std::io::Result<()> {
    let mut octets = fs::read("path/to/font.fnt")?;

    let font = BMFont::from_octets(&octets)?;

    println!("Font info: {:?}", font.info);
    println!("Character count: {}", font.chars.len());

    Ok(())
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
