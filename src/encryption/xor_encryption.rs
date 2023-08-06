use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub fn xor_enc(data: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut xor_data: Vec<u8> = Vec::with_capacity(data.len());
    for byte in data {
        xor_data.push(byte ^ key);
    }
    return xor_data;
}

pub fn xor_util(input_file_path: &String, output_file_path: &String, key: u8) {
    let mut input_data: Vec<u8> = Vec::new();
    BufReader::new(File::open(&input_file_path).unwrap())
        .read_to_end(&mut input_data).unwrap();
    BufWriter::new(File::create(output_file_path).unwrap())
        .write_all(&xor_enc(&input_data, key)).unwrap();
}