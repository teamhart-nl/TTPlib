use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use std::env;



fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let cmudict = BufReader::new(File::open("./cmudict/cmudict.dict").unwrap());
    let lines = cmudict.lines();
    let mut map = phf_codegen::Map::new();
    for lines in lines {
        let line = lines.unwrap();
        let mut split = line.split_whitespace();
        let word = split.next().unwrap();
        let phonemes = split.collect::<Vec<&str>>().join(" ");
        let phonemes = phonemes.replace(|c: char| c.is_numeric(), "");
        
        map.entry(word.to_string(), ("\"".to_string() + &phonemes + "\"").as_str());
    }

    write!(
        &mut file,
        "static KEYWORDS: phf::Map<&'static str, &'static str> = {}",
        map.build()
    )
    .unwrap();
    writeln!(&mut file, ";").unwrap();






}
