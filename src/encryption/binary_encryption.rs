use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::num::ParseIntError;
use std::str;

pub fn binary_enc(data: Vec<u8>) -> String {
    if data.is_empty() {
        println!("Fatal error\n");
    }
    let parsed_data = str::from_utf8(&data).unwrap().to_string(); //formatting bytes without string parse bad idea :skull:
    let mut binary_data = String::default();
    for character in parsed_data.clone().into_bytes() {
        binary_data += &format!("0{:b} ", character);
    }
    return binary_data;
}

pub fn binary_dec(data: Vec<u8>) -> String {
   let s_data = str::from_utf8(&data).unwrap();
    let dec_data = String::from_utf8(data).unwrap();
    return dec_data;
}

pub fn dec_util(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(9)
        .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
        .collect()
}

pub fn binary_util(input_file_path: &String, output_file_path: &String, should_dec: bool) {
    let input_file = File::open(&input_file_path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(input_file);
    let mut input_data: Vec<u8> = Vec::new();
    let mut binary_data: String = String::new();

    if let Err(err) = reader.read_to_end(&mut input_data) {
        println!("Fatal error {err}\n");
    }

    if should_dec {
        binary_data = binary_dec(input_data);
    } else {
        binary_data = binary_enc(input_data);
    }

    let output_file = File::create(output_file_path).unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(output_file);

    if let Err(err) = writer.write_all(&binary_data.into_bytes()) {
        println!("Fatal error {err}\n");
    }
}