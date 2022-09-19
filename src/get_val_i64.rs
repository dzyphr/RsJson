use std::fs;
use std::{collections::{HashMap}, process::{exit}};
use crate::get_file;
use crate::Value;
pub fn get_val_i64(full_filename: String, key: String, data_field: String) -> i64
{
    let mut exists = false;
    let json_block: String;
    (json_block, exists) = get_file(full_filename.clone(), exists);
    let mut value = 0;
    match(exists)
    {
        false => no_file(full_filename.clone()),
        true => value = check_file(json_block, full_filename, key, data_field)
    }
    fn no_file(full_filename: String)
    {
        println!("expected json file was not found");
        fs::remove_file(full_filename.clone()).expect("error removing temp file");
    }
    fn check_file(json_block: String, full_filename: String, key: String, data_field: String) -> i64
    {
        let jObj: Vec<HashMap<String, Value>> = serde_json::from_str(&json_block).expect("error indexing into json object");
        let mut i = 0;
        let mut was_found = false;
        let mut old_value = 0;
        for (x, y) in  &jObj.clone()[0]
        {
            i = i + 1;
            if x == &key
            {
                old_value = y[0][data_field.clone()].as_i64().expect("error parsing data as i64");
                dbg!(old_value.clone());
                was_found = true;
            }
            if (&i == &jObj.clone()[0].len()) && (was_found == false)
            {
                println!("The selected Key was NOT found in the json object!");
            }
        }
        return old_value;
    }
    return value;
}

