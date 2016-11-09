use std::fs::File;
use std::io::{BufWriter, BufReader};

use super::bincode::SizeLimit;
use super::bincode::rustc_serialize::{decode_from, encode_into};
use super::rustc_serialize::{Decodable, Encodable};

pub fn read_data<T>(file_path: &str) -> T where T: Decodable {
  let mut reader = BufReader::new(File::open(file_path).expect(format!("File \"{}\" is not found.", file_path).as_str()));
  let decoded: T = decode_from(&mut reader, SizeLimit::Infinite).expect(format!("File decoding error.").as_str());
  return decoded;
}

pub fn write_data<T>(file_path: &str, data: T) where T: Encodable {
  let mut writer = BufWriter::new(File::open(file_path).expect(format!("File \"{}\" is not found.", file_path).as_str()));
  encode_into(&data, &mut writer, SizeLimit::Infinite).expect(format!("File encoding error.").as_str());
}