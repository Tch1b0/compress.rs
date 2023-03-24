use crate::general::{Bytes, Cluster};
use std::{fs, collections::HashMap};

#[derive(Debug)]
pub enum CompressionError {
    InvalidPath,
    WritingError
}

type OccurenceMap = HashMap<Cluster, u16>;
fn create_occurence_map(data: &Vec<u8>) -> OccurenceMap {
    let mut m: OccurenceMap = HashMap::new();

    for i in (0..data.len()).step_by(4) {
        if i+3 >= data.len() { continue }

        let x = &data[i..=i+3];
        let key: Cluster = Cluster::from(x);

        match m.get_key_value(&key) {
            Some(v) => {
                m.insert(key, *v.1 + 1);
            },
            None => {
                m.insert(key, 1);
            }
        }
    }

    m
}

type Trend = Vec<(Cluster, u16)>;
fn analyze_trend(occurences: OccurenceMap) -> Trend {
    let mut x: Vec<_> = occurences.into_iter().collect();
    // sort the sequences by occurences
    x.sort_by_key(|v| v.1);
    x = x.into_iter().rev().collect();

    // only give a max of 254 keys, because the code will be stored in a byte and null is reserved
    if x.len() > 254 {
        x.drain(254..).collect()
    } else {
        x
    }
}

fn build_head(trend: &Trend) -> Bytes {
    let mut head: Bytes = vec![];
    for i in 1..=trend.len() as u8 {
        // push the "code" for the sequence
        head.push(i);
        
        // push the sequence one by one
        let sequence = &trend[i as usize - 1].0.deconstruct();
        for val in sequence {
            head.push(*val);
        }
    }
    
    head
}

fn build_body(content: &Bytes, trend: &Trend) -> Bytes {
    let mut body: Bytes = vec![];
    for i in 0..content.len() {
        if i % 4 != 0 { continue }
        
        let slice = &content[i..=std::cmp::min(i+3, content.len() - 1)];
        let trending_idx: Option<usize> = match slice.len() {
            4 => trend.into_iter().position(
                |(c, _)| *c == (Cluster::from(slice))
            ),
            _ => None,
        };

        match trending_idx {
            // append the sequence code
            // index offset by one because 0 is reserved
            Some(idx) => body.push((idx + 1) as u8),
            None => {
                // set null byte to indicate that incoming sequence should be interpreted literal
                body.push(0);
                body.extend_from_slice(slice);
            }
        }
    }

    body
}

pub fn compress(src: String, dest: String) -> Result<usize, CompressionError> {
    let raw_content = fs::read(src);
    if raw_content.is_err() {
        return Err(CompressionError::InvalidPath);
    }

    let content = raw_content.unwrap();
    let trend = analyze_trend(create_occurence_map(&content));
    let head = build_head(&trend);
    let body = build_body(&content, &trend);

    // init the compressed file with the head
    let mut compressed: Vec<u8> = head;
    // seperate head from body with a null byte
    compressed.push(0);
    // extend the compressed content with the body part
    compressed.extend(body);
    println!("{compressed:?}");
 
    match fs::write(dest, &compressed) {
        Err(_) => Err(CompressionError::WritingError),
        Ok(_) => Ok(compressed.len())
    }
}
