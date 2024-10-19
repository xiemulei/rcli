mod csv;
mod genpass;
mod base64;

use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;
pub use self::{
    base64::{Base64SubCommand, Base64Format},
    csv::OutputFormat, };

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}





