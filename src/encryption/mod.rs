pub mod xor_encryption {
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read, Write};

    pub fn xor_enc(data: Vec<u8>, key: u8) -> Vec<u8> {
        let mut xor_data: Vec<u8> = Vec::with_capacity(data.len());
        for byte in data {
            xor_data.push(byte ^ key);
        }
        return xor_data;
    }

    pub fn xor_util(input_path: &String, output_path: &String, key: u8) {
        let mut input_data: Vec<u8> = Vec::new();
        BufReader::new(
            File::open(&input_path).unwrap()
        ).read_to_end(&mut input_data).unwrap();
        BufWriter::new(
            File::create(output_path).unwrap()
        ).write_all(&xor_enc(input_data, key)).unwrap();
    }
}

pub mod binary_encryption {
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
        let dec_data: String = String::from_utf8(binary_to_decimal(&dec_util(str::from_utf8(&data).unwrap()
            .to_string())).unwrap()).unwrap();
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
        BufReader::new(File::open(&input_file_path).unwrap())
            .read_to_end(&mut input_data).unwrap();

        match should_dec {
            true => { binary_data = binary_dec(input_data); }
            false => { binary_data = binary_enc(input_data); }
        }

        BufWriter::new(File::create(output_file_path).unwrap())
            .write_all(&binary_data.into_bytes()).unwrap();
    }
}

pub mod base64_encryption {
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
}