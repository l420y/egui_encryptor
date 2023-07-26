use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub fn binary_enc(data: Vec<u8>) -> String {
    let mut binary_data = String::default();

    for character in data {
        binary_data += &format!("0{:b} ", character);
    }
    return binary_data;
}

pub fn binary_util(input_file_path: &String, output_file_path: &String) {
    let input_file = File::open(&input_file_path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(input_file);
    let mut input_data: Vec<u8> = Vec::new();

    if let Err(err) = reader.read_to_end(&mut input_data) {
        println!("Fatal error {err}\n");
    }

    let binary_data = binary_enc(input_data);

    let output_file = File::create(output_file_path).unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(output_file);

    if let Err(err) = writer.write_all(&binary_data.into_bytes()) {
        println!("Fatal error {err}\n");
    }
}