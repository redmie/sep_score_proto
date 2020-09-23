use std::io::Read;
pub mod tree;
pub mod parser;
pub mod scorer;
fn main() {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).expect("Cannot read stdin");
    /*
    let decoded_utf = from_utf8(&buffer)
        .expect("Error while transforming decoded data to utf8 string.");
    */
    let mut acc = 0;
    for line in buffer.split('\n') {
        let mut prepared_line = line.to_string();
        if let Some(position) = line.find(":") {
            let mut iter = prepared_line.chars();
            iter.by_ref().nth(position);
            prepared_line = iter.as_str().to_string();
        }
        prepared_line = prepared_line.to_lowercase().replace(" ", "").replace(" ", "");
        if prepared_line != "" {
            let tree = match parser::top_parser(&prepared_line) {
                Ok(t) => t,
                Err(e) => { println!("{}", e); return;},
            };
            let score = scorer::evaluate(&tree);
            println!("Tree: {:?} Line: {:?} Score: {:?}", tree, prepared_line, score);
            acc += score;
        }
    }
    println!("Total Score: {:?}", acc);
}
