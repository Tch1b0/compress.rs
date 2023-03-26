use std::{fs, collections::HashMap};

use crate::general::Bytes;

#[derive(Debug)]
pub enum DecompressionError {
    InvalidPath,
    InvalidFormat,
    WritingError
}

/** find the index of the head end */
fn find_head_end(content: &Bytes) -> Option<usize> {
    for (idx, val) in content.into_iter().step_by(5).enumerate() {
        if *val == 0 {
            return Some(idx * 5);
        }
    }

    None
}

/** build a map from the head */
fn build_map(head: &Bytes) -> HashMap<u8, &[u8]> {
    let mut m: HashMap<u8, &[u8]> = HashMap::new();
    for (idx, code) in head.into_iter().step_by(5).enumerate() {
        m.insert(*code, &head[(idx*5)+1..(idx*5)+5]);
    }

    m
}

/** decompress file from source into destination file */
pub fn decompress(src: String, dest: String) -> Result<usize, DecompressionError> {
    let raw_compressed = fs::read(src);
    if raw_compressed.is_err() {
        return Err(DecompressionError::InvalidPath);
    }
    let compressed = raw_compressed.unwrap();
    let head_end_op = find_head_end(&compressed);
    if head_end_op.is_none() {
        return Err(DecompressionError::InvalidFormat)
    }

    let head_end = head_end_op.unwrap_or(0);
    let head: &Bytes = &compressed[..head_end].into();
    let head_map = build_map(head);
    let cmp_body = &compressed[head_end+1..];

    let mut body: Bytes = vec![];
    let mut idx: usize = 0;

    while idx < cmp_body.len() {
        let val = cmp_body[idx];
        if val == 0 {
            
            // 0b00000000 0b10010001 0b00111010 0b10101100 0b00111100 ...
            // idx        idx + 1    idx + 2    idx + 3    idx + 4    idx + ...
            body.extend_from_slice(&cmp_body[idx+1..std::cmp::min(idx+5, cmp_body.len())]);
            idx += 5;
        } else {

            // 0b00001100
            // idx
            body.extend_from_slice(match head_map.get(&val) {
                Some(v) => *v,
                None => panic!("At pos {idx}: There is no key -> {val}"),
            });
            idx += 1;
        }
    }

    match fs::write(dest, &body) {
        Err(_) => Err(DecompressionError::WritingError),
        Ok(_) => Ok(body.len())
    }
}
