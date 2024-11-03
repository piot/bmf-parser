use std::fs;

#[test]
fn test() {
    let octets = fs::read("assets/menu.fnt").unwrap();

    let bmf = bmf_parser::BMFont::from_octets(&octets).expect("could not read menu.fnt");

    println!("{bmf:?}");
}
