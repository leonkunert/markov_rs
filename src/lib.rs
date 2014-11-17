use std::io::File;
use std::collections::TreeMap;

fn load_content_from_file(path: &Path) -> String {
    match File::open(path).read_to_string() {
        Ok(c)  => c,
        Err(e) => panic!("failed because of {}", e),
    }
}

#[test]
fn test_load_from_file() {
    let file_content = load_content_from_file(&Path::new("./test"));
    let mut table: TreeMap<String, TreeMap<String, uint>> = TreeMap::new();
    let mut previous_word: String = "".to_string();
    for word in file_content.split_str(" ".as_slice()) {
        if !table.contains_key(&word.into_string()) {
            table.insert(word.to_string(), TreeMap::new());
        }
        if previous_word.as_slice() != "".as_slice() {
            let mut val = table.get_mut(&previous_word.to_string()).unwrap();
            if val.contains_key(&word.to_string()) {
                let count = val.remove(&word.to_string()).unwrap();
                val.insert(word.to_string(), count.add(&1u));
            } else {
                val.insert(word.to_string(), 1u);
            }
        }
        previous_word = word.to_string();
    }
    for (key, value) in table.iter(){
        for (key_key, value_key) in value.iter() {
            if value_key > &3u {
                println!("\"{} {}\" occours {} times", key, key_key, value_key);
            }
        }
    }
    assert!(false);
}