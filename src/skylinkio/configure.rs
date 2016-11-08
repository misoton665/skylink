use skylinkio::rustc_serialize::*;
use domain::sky_environment::*;
use skylinkio::data_file_io::{read_data, write_data};

const config_dir: &'static str = "~/.skylink/";
const config_filename: &'static str = "config.bin";

#[derive(RustcDecodable, RustcEncodable, PartialEq)]
pub struct Configure {
  sky_env: SkyEnvironment,
}

pub fn default_configure_dir() -> &'static str {
  return config_dir;
}

pub fn read_configure(conf_dir: &str) -> Configure {
  let file_path = format!("{}{}", conf_dir, config_filename);
  let configure: Configure = read_data(&file_path.as_str());
  return configure;
}

pub fn write_configure(conf_dir: &str, configure: Configure) {
  let file_path = format!("{}{}", conf_dir, config_filename);
  write_data(&file_path.as_str(), configure);
}