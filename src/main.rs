#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod overwrite_data;
use crate::overwrite_data::overwrite_data;
mod get_file;
use crate::get_file::get_file;
mod init_json;
use crate::init_json::{init_json, Json_Structures};
mod make_first_entry;
use crate::make_first_entry::make_first_entry;
mod make_entry;
use crate::make_entry::make_entry;
mod do_if_logic;
use crate::do_if_logic::do_if_logic;
mod update_entry;
use crate::update_entry::update_entry;
extern crate serde_json;
use serde_json::Value;
use std::{collections::{HashMap}, process::{exit}};
use crate::Operations::{ADD, SUB, MTP, DIV};
use crate::Logic::{True, False};
use crate::LinearFn::{MakeEntry, UpdateEntry};
/*#[derive(Clone)]
pub struct Json_Structures
{
    pub outer: String,
    pub middle: String,
    pub middle_top_down: String,
    pub new_inner: String, //this implementation uses one data field and adds new entrys at the top,
    pub new_inner_top_down: String, //
    pub final_inner: String //later implementations will address more complex directionality
}*/
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
    True,
    False
}
#[derive(Clone)]
pub enum LinearFn
{
    MakeEntry,
    UpdateEntry
}

pub fn main() 
{
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
    //format:
    //json body outer format : [{     }]
    ////middle format "":
    /////inner format [{"": n}],
    //rules:
    //last inner cannot have comma at the end
    //specify data type, numbers surrounded by (') are characters, (") "defines a string"
    //
    let mut exists: bool = false;
    let filename = "testjson";
    let ext = ".json";
    let full_filename = filename.to_owned() + ext;
    let mut json_block: String;
    (json_block, exists)  = get_file(full_filename.clone(), exists);
    dbg!(exists);
    /*json_block = init_json(jstructure.clone());
    json_block = make_first_entry(
        json_block.clone().to_string(), "key0".to_string(), "data".to_string(), "6".to_string()
    );
    json_block = make_entry(
        json_block.clone().to_string(), "key1".to_string(), "data".to_string(), "420".to_string(), jstructure.clone(), top_down
    );
    json_block = make_entry(
        json_block.clone().to_string(), "key420".to_string(), "data".to_string(), "4233330".to_string(), jstructure.clone(), top_down
    );*/
    //json_block = update_entry(json_block.clone(), "key1".to_string(), "data".to_string(), jstructure.clone(), Some(ADD), Some(1));
    //overwrite_data(json_block.clone(), full_filename);
    let mut keys = Vec::new();
    keys.push("key0".to_string());
    keys.push("key1".to_string());
    keys.push("key420".to_string());
    json_block = do_if_logic(
        json_block.clone(), 
        keys, True, 
        UpdateEntry, None,  
        "data".to_string(), 
        "0".to_string(), 
        jstructure, 
        top_down,
        Some(ADD), Some(1));
    overwrite_data(json_block.clone(), full_filename);
    dbg!(json_block.clone());
}

