use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct AsciiReference {
    offset: u32,
    ascii: String
}

impl AsciiReference {
    pub fn new() -> Self {
        AsciiReference {
            offset: 0,
            ascii: String::from("")
        }
    }
    
    pub fn from(offset: u32, ascii: String) -> Self {
        AsciiReference {
            offset,
            ascii
        }
    }
}

pub type AsciiReferences = Vec<AsciiReference>;

pub fn extract_ascii_references(bytes: &Vec<u8>, min_chars:u8) -> AsciiReferences {
    let mut offset:u32 = 0;
    let mut possible_str:String = String::from("");
    let mut refs:AsciiReferences = AsciiReferences::new();

    for byte in bytes.iter() {
        if *byte >= 32 && *byte <=126 {
            possible_str.push(*byte as char);
        }else{
            if possible_str.len() > min_chars.into() {
                //let entropy=calculate_entropy(&possible_str);
                //println!("{} -> {}",possible_str,entropy);
                refs.push(AsciiReference::from(offset, possible_str));
            }
            possible_str= String::from("");
        }
        offset+=1;
    }

    refs
}

fn calculate_entropy(input: &str) -> f64 {
    let length = input.len();
    let mut char_count_map: HashMap<char, usize> = HashMap::new();

    // Conta la frequenza di ogni carattere nella stringa
    for c in input.chars() {
        *char_count_map.entry(c).or_insert(0) += 1;
    }

    let mut entropy = 0.0;
    // Calcola l'entropia basata sulla frequenza dei caratteri
    for &count in char_count_map.values() {
        let frequency = count as f64 / length as f64;
        entropy -= frequency * (frequency.log2());
    }

    entropy
}
