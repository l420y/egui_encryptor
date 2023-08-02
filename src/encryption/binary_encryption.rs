use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::str;

use ascii_converter::*;

pub fn binary_enc(data: Vec<u8>) -> String {
    let mut binary_data: String = String::default();
    if !data.is_empty() {
        for character in data {
            binary_data += &format!("{:b} ", character);
        }
    }
    return binary_data;
}

pub fn binary_dec(data: Vec<u8>) -> String {
    let parsed_data: String = str::from_utf8(&data).unwrap().to_string();
    let dec_data: String = String::from_utf8(binary_to_decimal(&dec_util(parsed_data)).unwrap()).unwrap();
    return dec_data;
}

pub fn dec_util(s: String) -> Vec<u32> {
    let mut bin_vec: Vec<u32> = Vec::new();
    if !s.chars().all(char::is_alphabetic) {
        for bytes in s.split_whitespace() {
            bin_vec.push(bytes.parse::<u32>().unwrap());
        }
    }
    return bin_vec;
}

pub fn binary_util(input_file_path: &String, output_file_path: &String, should_dec: bool) {
    let mut input_data: Vec<u8> = Vec::new();
    let mut binary_data: String = String::new();
    let mut reader: BufReader<File> = BufReader::new(File::open(&input_file_path).unwrap());
    reader.read_to_end(&mut input_data).unwrap();

    match should_dec {
        true => { binary_data = binary_dec(input_data); }
        false => { binary_data = binary_enc(input_data); }
    }

    let mut writer: BufWriter<File> = BufWriter::new(File::create(output_file_path).unwrap());
    writer.write_all(&binary_data.into_bytes()).unwrap();
}