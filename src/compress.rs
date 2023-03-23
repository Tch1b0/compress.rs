use crate::general::Bytes;
use std::{fs, collections::HashMap};

#[derive(Debug)]
pub enum CompressionError {
    InvalidPath,
    WritingError
}

#[derive(Eq, Hash, PartialEq)]
struct Cluster{
    value: u32,
}

impl Cluster {
    pub fn deconstruct(&self) -> [u8; 4] {
        print!("{:b} ", self.value);
        let mut arr: [u8; 4] = [0; 4];
        // lower/big endian issues, so reverse array
        let bytes: [u8; 4] = unsafe { std::mem::transmute(self.value) };
        for i in 0..=3 {
            arr[i] = bytes[3 - i];
        }

        arr
    }
}

impl From<&[u8]> for Cluster {
    fn from(vals: &[u8]) -> Self {
        Cluster {
            value: u32::from_be_bytes(vals.try_into().expect("slice has incorrect length"))
        }
    }
}

type OccurenceMap = HashMap<Cluster, u16>;
fn create_occurence_map(data: &Vec<u8>) -> OccurenceMap {
    let mut m: OccurenceMap = HashMap::new();
    
    for i in 0..data.len() {
        if i % 4 != 0 || i+3 >= data.len() { continue }

        let x = &data[i..=i+3];
        let key: Cluster = Cluster::from(x);
        if m.contains_key(&key) {
            let (_, v) = m.get_key_value(&key).unwrap();
            m.insert(key, v + 1);
        } else {
            m.insert(key, 1);
        }
    }

    m
}

type Trend = Vec<(Cluster, u16)>;
fn analyze_trend(occurences: OccurenceMap) -> Trend {
    let mut x: Vec<_> = occurences.into_iter().collect();
    x.sort_by_key(|v| v.1);
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
            print!(":{:8b}:", val);
            head.push(*val);
        }
        println!();
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
            // index offset by one because 0 is reserved
            Some(idx) => body.push((idx + 1) as u8),
            None => {
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

    let mut compressed: Vec<u8> = head;
    compressed.push(0);
    compressed.extend(body);
 
    match fs::write(dest, &compressed) {
        Err(_) => Err(CompressionError::WritingError),
        Ok(_) => Ok(compressed.len())
    }
}
