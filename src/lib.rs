#![deny(clippy::all)]

use silk_rs::encode_silk;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn encode(input: String, output: String, sample_rate: i32) {
  let input_data = std::fs::read(input).unwrap();
  let output_data = encode_silk(input_data, sample_rate, 25000, true).unwrap();
  std::fs::write(output, &output_data[0..output_data.len() - 1]).unwrap();
}