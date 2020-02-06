mod binary_chunk;
mod vm;
use std::fs::File;
use std::io;
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let mut file = File::open("../tests/param_num.out")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    // undump
    let mut r = binary_chunk::reader::Reader::new(data);
    r.check_header();
    r.read_byte(); // size_upvalues
    let result = r.read_proto(String::from(""));
    // print
    r.print_content(&result);
    Ok(())
}
