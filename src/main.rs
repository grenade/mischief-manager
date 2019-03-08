extern crate clap;
mod minion;
use minion::{spawn/*,slay*/};
use clap::{Arg, App};

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const CARGO_PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
  let matches = App::new(CARGO_PKG_NAME)
    .version(CARGO_PKG_VERSION)
    .author(CARGO_PKG_AUTHORS)
    .about(CARGO_PKG_DESCRIPTION)
    .arg(Arg::with_name("worker type")
      .required(false)
      .takes_value(true)
      .index(1)
      .help("worker type to deploy"))
    .get_matches();
  let worker_type = matches.value_of("worker type").unwrap();
  spawn(worker_type.to_string());
}
