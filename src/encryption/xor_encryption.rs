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
    let mut reader: BufReader<File> = BufReader::new(File::open(&input_file_path).unwrap());
    let mut input_data: Vec<u8> = Vec::new();
    reader.read_to_end(&mut input_data).unwrap();
    let xor_data: Vec<u8> = xor_enc(&input_data, key);
    let mut writer: BufWriter<File> = BufWriter::new(File::create(output_file_path).unwrap());
    writer.write_all(&xor_data).unwrap();
}