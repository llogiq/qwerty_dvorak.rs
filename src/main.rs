use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

macro_rules! hash {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut tmp_hash = HashMap::new();
            $(
                tmp_hash.insert($k, $v);
            )*
            tmp_hash
        }
    };
}

fn qd_map() -> HashMap<char, char> {
    hash!(
        'a' => 'a', 'A' => 'A', 'b' => 'x', 'B' => 'X',    
        'c' => 'j', 'C' => 'J', 'd' => 'e', 'D' => 'E',    
        'f' => 'u', 'F' => 'U', 'g' => 'i', 'G' => 'I',    
        'h' => 'd', 'H' => 'D', 'i' => 'c', 'I' => 'C',    
        'j' => 'h', 'J' => 'H', 'k' => 't', 'K' => 'T',
        'l' => 'n', 'L' => 'N', 'm' => 'm', 'M' => 'M',    
        'n' => 'b', 'N' => 'B', 'o' => 'r', 'O' => 'R',    
        'p' => 'l', 'P' => 'L', 'r' => 'p', 'R' => 'P',    
        's' => 'o', 'S' => 'O', 't' => 'y', 'T' => 'Y',    
        'u' => 'g', 'U' => 'G', 'v' => 'k', 'V' => 'K',    
        'x' => 'q', 'X' => 'Q', 'y' => 'f', 'Y' => 'F'
     )
}

fn open_dict() -> File {
    let path = Path::new("/usr/share/dict/words");
    let display = path.display();
    
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file
    };
    file
}

fn parse_words<'a>(dict_file: &mut File, dict_contents: &'a mut String) -> HashSet<String> {   
    let words = match dict_file.read_to_string(dict_contents) {
        Ok(_) => {            
            dict_contents.lines().map(|line| line.trim_right().to_string()).collect::<HashSet<String>>()
        }
        Err(_) => panic!("couldn't parse dict file")
    };
    words
}

fn main() {
    // Parse dictionary file
    let mut dict_file = open_dict();
    let mut s = String::new();
    let words = parse_words(&mut dict_file, &mut s);

    // Words with e, q, w, and z will be invalid when converted
    // to Dvorak because those positions are special characters
    let valid_words: HashSet<&String> = words.iter().filter(|word| 
        !word.contains("q") && !word.contains("Q") &&
        !word.contains("w") && !word.contains("W") &&
        !word.contains("e") && !word.contains("E") && 
        !word.contains("z") && !word.contains("Z")
    ).collect();

    // Qwerty -> Dvorak conversion map
    let qd_map = qd_map();

    let mut dvorak_words: HashSet<String> = HashSet::new();  
    for word in &valid_words {
        let dvorak_word: String = word.chars().map(|c| qd_map[&c]).collect();
        dvorak_words.insert(dvorak_word);
    }

    println!("{:?}", words.intersection(&dvorak_words).collect::<Vec<&String>>());
}
