use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use ascii_converter::decimals_to_string;
use base64::{Engine as _, engine::general_purpose};

pub fn base64_enc(data: Vec<u8>) -> String {
    return general_purpose::STANDARD_NO_PAD.encode(data);
}

pub fn base64_dec(str: String) -> String {
    return decimals_to_string(&general_purpose::STANDARD_NO_PAD.decode(str).unwrap()).unwrap();
}

pub fn base64_util(input_file_path: &String, output_file_path: &String, should_dec: bool) {
    let mut input_data: Vec<u8> = Vec::new();
    let mut base64_data: String = String::new();
    BufReader::new(File::open(&input_file_path).unwrap())
        .read_to_end(&mut input_data).unwrap();

    match should_dec {
        true => { base64_data = base64_dec(decimals_to_string(&input_data).unwrap()); }
        false => { base64_data = base64_enc(input_data); }
    }

    BufWriter::new(File::create(output_file_path).unwrap())
        .write_all(&base64_data.into_bytes()).unwrap();
}