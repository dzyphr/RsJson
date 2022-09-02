#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

extern crate serde_json;
use serde_json::Value;
use std::{path::Path, fs::{File}, io::{Read, Write}, collections::{HashMap}, process::{exit}};
use crate::Operations::{ADD, SUB, MTP, DIV};
use crate::Logic::{True, False};
use crate::LinearFn::{MakeEntry, UpdateEntry};
#[derive(Clone)]
pub struct Json_Structures
{
    pub outer: String,
    pub middle: String,
    pub middle_top_down: String,
    pub new_inner: String, //this implementation uses one data field and adds new entrys at the top,
    pub new_inner_top_down: String, //
    pub final_inner: String //later implementations will address more complex directionality
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
    True,
    False
}

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
   /* json_block = init_json(jstructure.clone());
    json_block = make_first_entry(
        json_block.clone().to_string(), "key0".to_string(), "data".to_string(), "6".to_string()
    );
    json_block = make_entry(
        json_block.clone().to_string(), "key7".to_string(), "data".to_string(), "420".to_string(), jstructure, top_down
    );*/
    json_block = make_entry(
        json_block.clone().to_string(), "key420".to_string(), "data".to_string(), "4233330".to_string(), jstructure, top_down
    );
    //json_block = update_entry(json_block.clone(), "key1".to_string(), "data".to_string(), jstructure, ADD, 1);
    //overwrite_data(json_block.clone(), full_filename);
    /*let mut keys = Vec::new();
    keys.push("key0".to_string());
    keys.push("key1".to_string());
    keys.push("key2".to_string());
    json_block = do_if_logic(
        json_block.clone(), 
        keys, True, 
        UpdateEntry, None,  
        "data".to_string(), 
        "0".to_string(), 
        jstructure, 
        top_down,
        Some(ADD), Some(1));*/
    overwrite_data(json_block.clone(), full_filename);
    dbg!(json_block.clone());
}

pub fn overwrite_data(json_block: String, full_filename: String)
{
    let mut  file = File::create(full_filename.clone()).expect("error opening json file");
    file.write_all(&json_block.as_bytes()).expect("error writing to selected json file");
}

pub fn get_file(full_filename: String, mut exists: bool) -> (String, bool)
{
    let mut filecontents = String::new();
    if Path::new(&full_filename).exists()
    {
        let mut file = File::open(&full_filename).expect("unable to open json file");
        file.read_to_string(&mut filecontents).expect("unable to read from json file");
        exists = true;
    }
    else
    {
        File::create(full_filename).expect("error creating new json file");
        exists = false;
    }
    return (filecontents, exists);
}

//init json and make first entry are json-direction 'agnostic' meaning they are unbiased to the
//future choice of your json structure as they only are called to make the first entry...

pub fn init_json( jstructure: Json_Structures ) -> String
{
    let mut outer_structure = jstructure.outer;
    let mut middle_structure = jstructure.middle;
    let final_inner_structure = jstructure.final_inner;
    middle_structure.insert_str(4, &final_inner_structure);//start with the final inner structure
                                                           //add a new one on every entry to
                                                           //the top
    outer_structure.insert_str(3 , &middle_structure);
    let json_block = outer_structure.clone();
    //dbg!(&json_block);
    return json_block;
}

pub fn make_first_entry(mut json_block: String, key: String, data:  String, n: String) -> String
{
    let t_len = key.len();
    let d_len = data.len();
    json_block.insert_str(5 ,&key);//only accounting for starting block aka outer structure
    json_block.insert_str(13+t_len ,&data);//accounting for key added in this step
    json_block.insert_str(16+d_len+t_len ,&n);//accounting for key and data here
    return json_block;
}

pub fn make_entry(mut json_block: String, key: String, data:  String, n: String, jstructure: Json_Structures, top_down: bool) -> String
{
    let t_len = key.len();
    let d_len = data.len();
    let mut new_inner_structure: String;
    let mut middle_structure: String;
    if top_down == false
    {
        new_inner_structure = jstructure.new_inner;
        middle_structure = jstructure.middle;
    }
    else
    {
        new_inner_structure = jstructure.new_inner_top_down;
        middle_structure = jstructure.middle_top_down;
    }
    new_inner_structure.insert_str(6, &data);
    new_inner_structure.insert_str(9+d_len, &n);
    println!("{}", middle_structure.clone());
    if top_down == true
    {
        middle_structure.insert_str(4, &key);//shifted both up by two
        middle_structure.insert_str(6+t_len, &new_inner_structure);
    }
    else
    {
        middle_structure.insert_str(2, &key);//shifted both up by two
        middle_structure.insert_str(4+t_len, &new_inner_structure);
    }
    println!("{}", middle_structure.clone());
    if top_down == false
    {
        json_block.insert_str(3, &middle_structure);    
    }
    else
    {
        json_block.insert_str((json_block.len() - 3), &middle_structure)
    }
        //thought: to do top down json just insert this in (json_block.len() - 3) and have \n 
                                                //at the start of the new entry aka middle
                                                //structure. This also means that starting from
                                                //first entry, additional entrys will add a comma
                                                //to the end of the previous entry instead of at
                                                //the end of the new entry as will be the default
                                                //case*/
    return json_block;    
}

pub fn do_if_logic
(
    mut json_block: String, 
    keys: Vec<String>, 
    qualifier: Logic, 
    qualified_function: LinearFn, 
    disqualified_function: Option<String>,
    data: String, 
    n: String, 
    jstructure: Json_Structures,
    top_down: bool,
    opt: Option<Operations>,
    opt_val: Option<i64>
) -> String
{
    let bCheck: bool;
    match qualifier
    {
        True => bCheck = true,
        False => bCheck = false,
    }
    let mut keyVals: HashMap<String, bool> = HashMap::new();
    for i in 0..keys.len()
    {
        keyVals.insert(keys[i].clone(), false);//initialize the hashmap
    }
    let jObj: Vec<HashMap<String, Value>> = serde_json::from_str(&json_block).expect("error indexing into json object");
    //let mut i = 0;
    for (x, y) in  &jObj.clone()[0]
    {
        //i = i + 1;
        if keyVals.contains_key(x)
        {
            keyVals.insert(x.to_string(), true);
        }
    }
    dbg!(keyVals.clone());
    for (x, y) in keyVals
    {
        if y == bCheck
        {
            match qualified_function
            {
                MakeEntry => json_block = make_entry(
                    json_block.clone(), 
                    x.clone(), 
                    data.clone(), 
                    n.clone(), 
                    jstructure.clone(),
                    top_down.clone()
                ),
                UpdateEntry => json_block = update_entry(
                    json_block.clone(), 
                    x.clone(), 
                    data.clone(), 
                    jstructure.clone(), 
                    opt.clone(), 
                    opt_val.clone()
                )
            };   
        }
    }
    return json_block;
}



pub fn update_entry
(
    mut json_block: String, key: String, data:  String, jstructure: Json_Structures, opt: Option<Operations>, 
    opt_val: Option<i64>
) -> String
{
    let mut uopt: Operations = ADD;
    match opt
    {
        Some(ref Operations) => uopt = opt.clone().expect("error parsing operation"),
        None => println!("must provide an operation to call update entry function\nno operation provided")
    };
    let mut uopt_val: i64 = 0;
    match opt_val
    {
        Some(i64) => uopt_val = opt_val.expect("error parsing operation value"),
        None => println!("must provide an operation-value to modify original value with\n no operation-value provided")
    };
    let d_len = data.len();
    let mut old_value: i64 = 0;
    let jObj: Vec<HashMap<String, Value>> = serde_json::from_str(&json_block).expect("error indexing into json object");
    let mut i = 0;
    let mut was_found = false;
    for (x, y) in  &jObj.clone()[0]
    {
        i = i + 1;
        if x == &key
        {
            old_value = y[0][data.clone()].as_i64().expect("error parsing data as i64");
            was_found = true;
        }
        if (&i == &jObj.clone()[0].len()) && (was_found == false)
        {
            println!("The selected Key was NOT found in the json object!");
            exit(1);
        }
    }
    let new_val: i64;
    match uopt
    {
        ADD => new_val = add_opt(old_value, uopt_val),
        SUB => new_val = sub_opt(old_value, uopt_val),
        MTP => new_val = mtp_opt(old_value, uopt_val),
        DIV => new_val = div_opt(old_value, uopt_val)
    };
    pub fn add_opt(old_data: i64, uopt_val: i64) -> i64
    {
        return old_data + uopt_val;
    }
    pub fn sub_opt(old_data: i64, uopt_val: i64) -> i64
    {
        return old_data - uopt_val;
    }
    pub fn mtp_opt(old_data: i64, uopt_val: i64) -> i64
    {
        return old_data * uopt_val;
    }
    pub fn div_opt(old_data: i64, uopt_val: i64) -> i64
    {
        return old_data / uopt_val;
    }
    //let t_loc = json_block.find(&key).expect("cannot find Key");
    let mut middle_structure = jstructure.middle.clone();
    let mut final_inner_structure = jstructure.final_inner.clone();
    let compare_model: String;
    //this function should theoretically also be direction agnostic as it only replaces the part of
    //the entry that is contained within the comma seperators. theoretically as long as it modifys
    //json that was valid it should not need to know which direction to go. this could be added but
    //seems wasteful and doesn't really make much sense for large data structures that are just
    //going to be computed on anyway. If a human has to read the json output regularly perhaps
    //adding reordering updated entries would be useful? For example to view newest to latest
    //order.
    middle_structure.insert_str(2, &key);
    final_inner_structure.insert_str(6, &data);
    final_inner_structure.insert_str(9+d_len, &old_value.to_string());
    middle_structure.insert_str(middle_structure.len(), &final_inner_structure);
    compare_model = middle_structure;
    middle_structure = jstructure.middle;
    final_inner_structure = jstructure.final_inner;
    let new_model: String;
    middle_structure.insert_str(2, &key);
    final_inner_structure.insert_str(6, &data);
    final_inner_structure.insert_str(9+d_len, &new_val.to_string());
    middle_structure.insert_str(middle_structure.len(), &final_inner_structure);
    new_model = middle_structure;
    json_block = json_block.replace(&compare_model, &new_model);
    //dbg!(&json_block);
    return json_block;
}
