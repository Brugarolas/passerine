use passerine_derive::Inject;

use crate::common::data::Data;

/// Prints some data to stdout with a trailing newline.
pub fn println(data: Data) -> Result<Data, String> {
    println!("{}", data);
    return Ok(data);
}

/// Prints some data to stdout without a trailing newline.
pub fn print(data: Data) -> Result<Data, String> {
    print!("{}", data);
    return Ok(data);
}

pub fn to_string(data: Data) -> Result<Data, String> {
    Ok(Data::String(format!("{}", data)))
}

#[derive(Inject)]
pub struct Write(Data);

#[derive(Inject)]
pub struct Writeln(Data);

#[derive(Inject)]
pub struct Show(Data);
