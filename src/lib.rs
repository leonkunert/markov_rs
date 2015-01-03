use std::io::File;
use std::collections::BTreeMap;

fn main() {
    let chain = load_chain_from_file(&Path::new(".test"));
    println!("Chain {}", chain);
}


fn load_content_from_file(path: &Path) -> String {
    match File::open(path).read_to_string() {
        Ok(c)  => c,
        Err(e) => panic!("failed because of {}", e),
    }
}

fn load_chain_from_file(path: &Path) -> BTreeMap<String, BTreeMap<String, uint>> {
    let file_content = load_content_from_file(path);
    let mut table: BTreeMap<String, BTreeMap<String, uint>> = BTreeMap::new();
    let mut previous_word: String = "".to_string();
    for word in file_content.split_str(" ".as_slice()) {
        if !table.contains_key(&word.into_string()) {
            table.insert(word.to_string(), BTreeMap::new());
        }
        if previous_word.as_slice() != "".as_slice() {
            let mut val = table.get_mut(&previous_word.to_string()).unwrap();
            if val.contains_key(&word.to_string()) {
                let count = val.remove(&word.to_string()).unwrap();
                val.insert(word.to_string(), count + 1u);
            } else {
                val.insert(word.to_string(), 1u);
            }
        }
        previous_word = word.to_string();
    }
    table
}

fn iter_for_word(word: String, chain: BTreeMap<String, BTreeMap<String, uint>>) {
    println!("String {}", word);

}