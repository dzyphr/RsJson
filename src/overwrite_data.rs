use std::{fs::{File}, io::{Write}};
pub fn overwrite_data(json_block: String, full_filename: String)
{
    let mut  file = File::create(full_filename.clone()).expect("error opening json file");
    file.write_all(&json_block.as_bytes()).expect("error writing to selected json file");
}

