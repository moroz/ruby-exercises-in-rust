use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let file = File::open("../wordcount.txt")?;
    let f = BufReader::new(file);
    let mut counts: HashMap<String, i32> = HashMap::new();
    let split_regex = Regex::new("[.\\n\\s]+").unwrap();

    for line in f.lines() {
        for word in split_regex.split(&line.unwrap()) {
            if word.is_empty() {
                continue;
            }
            let word = word.to_lowercase();
            if let Some(count) = counts.get_mut(&word) {
                *count += 1;
            } else {
                counts.insert(word, 1);
            }
        }
    }

    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by_key(|&(_k, v)| v);
    sorted.reverse();

    for (k, v) in sorted {
        println!("{}: {}", k, v);
    }

    Ok(())
}
