#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
pub mod overwrite_data;
use crate::overwrite_data::overwrite_data;
pub mod get_file;
use crate::get_file::get_file;
pub mod init_json;
use crate::init_json::{init_json};
pub mod make_first_entry;
use crate::make_first_entry::make_first_entry;
pub mod make_entry;
use crate::make_entry::make_entry;
pub mod do_if_logic;
use crate::do_if_logic::do_if_logic;
pub mod update_entry;
use crate::update_entry::update_entry;
extern crate serde_json;
use serde_json::Value;
use std::{collections::{HashMap}, process::{exit}};
use crate::Operations::{ADD, SUB, MTP, DIV};
use crate::Logic::{Found, NotFound};
use crate::LinearFn::{MakeEntry, UpdateEntry};
#[derive(Clone)]
pub struct Json_Structures
{
    pub outer: String,
    pub middle: String,
    pub middle_top_down: String,//these rely on styling choice top down or bottom up
    pub new_inner: String, //
    pub new_inner_top_down: String, //
    pub final_inner: String //
}
#[derive(Clone)]
pub enum Operations
{
    ADD,
    SUB,
    MTP, 
    DIV
}

pub enum Logic
{
    Found,
    NotFound
}
#[derive(Clone)]
pub enum LinearFn
{
    MakeEntry,
    UpdateEntry
}

pub fn json_ext(name: String) -> String
{
    return name.to_owned() + ".json";
}

pub fn main() 
{
    /*
     *
     * Rust Json Generator and Parser
     *
     * for modular updating editing and checking of json files with variable structure
     * usage:
     * Define your jstructure parts that you wish to build,
     * they must make up a valid json in the end(more designs will be implemented down the line).
     *
     * 1)
     * set up:
     *
     * set up a jstructure as shown below, custom ones can be made if following json style properly
     * set the filename parameter: optionally use the json_ext() function to add extentions to filenames
     * modularly, nevertheless store the FULL filename in a variable if you wish to continue to modify the file.
     * ALSO make an exists boolean, this will let you know if you are editing an existing file or not.
     * optionally set up a boolean called top_down that is true if you want top down json structure
     * or false if you want bottom up json structure.(referring to the direction in which keys are
     * added)
     *
     * 
     * 2) 
     * declare a new String variable to store the json file contents:
     *
     * 
     * 3) 
     * use the get_file() function: 
     *
     * this checks if the target filename exists already, if so it will read the contents into the
     * json_block String, otherwise it will create a new file with the target name.
     * Also pass exists to this function to view the status of the function. (may be optional in
     * future builds) 
     *
     * At this point, if your file exists you will likely wish to modify the existing contents, 
     * however you can easily overwrite them as well. If the file does not exist you will wish to
     * create it with a brand new json structure. This will probably be automatable down the line
     * via more Logic enums 
     *
     * [Assuming that you are working with a new json] 
     *
     * //if your file already exists and you do this it will simply overwrite it as you specify it to
     *
     *
     * 4) 
     * use the init_json() function:
     *
     * this will build the chosen outer structure of the json file.
     *
     *
     * 5) 
     * use the make_first_entry() function:
     *
     * this will make the very first entry into the json file, it is just an entry with no 
     * comma at the end. it takes these parameters: 
     * 
     * json_block(the string to hold json contents), the name of the key you want to add to the
     * entry String,   the name of the subkey or data field that you want to enter a value in String, the value to
     * be entered into the data field i32. 
     *
     *
     * 6) 
     * use the make_entry() function:
     *
     * this can be used continuously as soon as the json has its first entry it will append the
     * next one properly. This one has the same parameters as make_first_entry() and
     * also requires you to pass in the jstructure & pass in the
     * top_down variable if you made one, otherwise just pass in true for top down or false for bottom up.
     *
     *
     * 7) 
     * use the overwrite_data function: 
     *
     * this will update your target json file. it takes the json_block string and the full filename
     * as arguments.
     *
     * [Assumes step 7 has been passed or the target json file exists]
     *
     *
     * 8)
     * use the update_entry() function:
     *
     * will update an existing entry, if the entry doesn't exist this wont work
     * this takes these parameters:
     * 
     * operation OPT, opt_val i32
     *
     * operations are defined in the OPT enum they are mathematical operations to execute on data
     * opt val is the i32 that we will use to modify the original data value
     * currently they are optional for a later function but the optional path will be 
     * abstracted away as they technically are mandatory for this function
     *
     *
     * 9)
     * use the do_if_logic() function:
     *
     * first get a vector full of the key names you wish to perform an operation on.
     * then call do if logic. this function is capable of executing commands on specific keys, and also comparing the keys
     * against the json string in order to perform logical operations on it. For example this allows us to call
     * certain functions on keys that exist and other functions on keys that dont exist yet.
     * This contains a few mandatory variables that are the common parameters
     * needed for each logical/linear function.
     *
     * This function takes these parameters:
     *
     * json_block String, keys Vector, qualifier Logic, qualifiedFn LinearFn, disqualifiedFn LinearFn(optional), 
     * datafield String, dataval String(optional), jstructure Json_Structures, top_down bool,
     * operation OPT(optional), optval i32(optional)
     *
     * the qualifier is a collection of keywords that will logically direct the command to check
     * for the specification(ex: Found will check that the key is found)
     * the qualified fn will execute on those keys, and the optional disqualified fn will execute
     * on the opposite case (ex: Found will make disqualified fn execute on keys that arent found)
     * 
     *
     * then repeat step 7 (overwrite_data()) to update the json file
     *
     * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */
    //json formatting guide(for current style):
    //json body outer format : [{     }]
    ////middle format "":
    /////inner format [{"": n}],
    //rules:
    //last inner cannot have comma at the end
    //specify data type, numbers surrounded by (') are characters, (") "defines a string"
    //  1)
    let top_down: bool = false;
    let jstructure = Json_Structures
    {
        outer: "[{\n\n}]".to_string(),
        middle: "\t\"\":".to_string(),
        middle_top_down: ",\n\t\"\":".to_string(),
        new_inner: "\n\t\t[{\"\": }],\n".to_string(),
        new_inner_top_down: "\n\t\t[{\"\": }]".to_string(),
        final_inner: "\n\t\t[{\"\": }]".to_string()
    };
    let mut exists: bool = false;
    let filename = "testjson";
    let full_filename = json_ext(filename.to_string());
    //  2)
    let mut json_block: String;
    //  3)
    (json_block, exists)  = get_file(full_filename.clone(), exists);
    dbg!(exists);
    //  4)
    json_block = init_json(jstructure.clone());
    //  5)
    json_block = make_first_entry(
        json_block.clone().to_string(), 
        "key0".to_string(), 
        "data".to_string(), 
        "1".to_string()
    );
    //  6)
    json_block = make_entry(
        json_block.clone().to_string(), 
        "key1".to_string(), 
        "data".to_string(), 
        "2".to_string(), 
        jstructure.clone(), top_down
    );
    json_block = make_entry(
        json_block.clone().to_string(), 
        "key2".to_string(), 
        "data".to_string(), 
        "3".to_string(), 
        jstructure.clone(), top_down
    );
    //  7)
    overwrite_data(json_block.clone(), full_filename.clone());
    //  8)
    json_block = update_entry(
        json_block.clone(), 
        "key0".to_string(), 
        "data".to_string(), 
        jstructure.clone(), 
        Some(ADD), Some(1)
    );
    //  9)
    let mut keys = Vec::new();
    keys.push("key1".to_string());
    keys.push("key2".to_string());
    json_block = do_if_logic(
        json_block.clone(), 
        keys, Found, 
        UpdateEntry, None,  
        "data".to_string(), 
        Some("0".to_string()), 
        jstructure, 
        top_down,
        Some(ADD), Some(1)
    );
    //  7)
    overwrite_data(json_block.clone(), full_filename.clone());
    dbg!(json_block.clone());
}

