use std::{fs, collections::HashMap};

use crate::general::Bytes;

pub enum DecompressionError {
    InvalidPath,
    InvalidFormat,
    WritingError
}

fn find_head_end(content: &Bytes) -> Option<usize> {
    for (idx, val) in content.into_iter().step_by(5).enumerate() {
        if *val == 0 {
            return Some(idx);
        }
    }

    None
}

fn build_map(head: &Bytes) -> HashMap<u8, &[u8]> {
    let mut m: HashMap<u8, &[u8]> = HashMap::new();
    for (idx, code) in head.into_iter().step_by(5).enumerate() {
        m.insert(*code, &head[idx+1..idx+5]);
    }

    m
}

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

    let head_end = head_end_op.unwrap();
    println!("{}", head_end);
    let x = &compressed[0..head_end].into();
    let head_map = build_map(x);
    let cmp_body = &compressed[head_end+1..compressed.len()];

    let mut body: Bytes = vec![];

    let mut idx = 0 as usize;
    while idx < cmp_body.len() {
        if cmp_body[idx] == 0 {
            body.extend_from_slice(&cmp_body[idx+1..idx+5]);
            idx += 5;
        } else {
            body.extend_from_slice(head_map.get(&cmp_body[idx]).unwrap());
            idx += 1;
        }
    }

    match fs::write(dest, &compressed) {
        Err(_) => Err(DecompressionError::WritingError),
        Ok(_) => Ok(body.len())
    }
}
