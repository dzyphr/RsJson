use serde::Deserialize;
use serde_json::{Result, Value};
use std::{path::Path, fs::{File, OpenOptions}, io::{Read, Write},};
fn main() 
{
    //format:
    //json body outer format : [{     }]
    ////middle format "":
    /////inner format [{"": n}],
    //rules:
    //last inner cannot have comma at the end
    //specify data type, numbers surrounded by (') are characters, (") "defines a string"
    //
    let filename = "testjson";
    let ext = ".json";
    let fullname = filename.to_owned() + ext;
    let mut json_block  = get_file(fullname.clone());
    json_block = init_json(json_block.to_string());
    json_block = make_first_entry(
        json_block.clone().to_string(), "title0".to_string(), "data".to_string(), "6".to_string()
    );
    json_block = make_entry(
        json_block.clone().to_string(), "title1".to_string(), "data".to_string(), "6".to_string()
    );
    overwrite_data(json_block, fullname);
}

fn overwrite_data(json_block: String, fullname: String)
{
    let mut  file = File::create(fullname.clone()).expect("error opening json file");
    file.write_all(&json_block.as_bytes());
    file.close();
}

fn get_file(fullname: String) -> String
{
    let mut filecontents = String::new();
    if Path::new(&fullname).exists()
    {
        let mut file = File::open(&fullname).expect("unable to open json file");
        file.read_to_string(&mut filecontents).expect("unable to read from json file");
    }
    else
    {
        let mut file = File::create(fullname).expect("error creating new json file");
    }
    return filecontents;
}

fn init_json(mut json_block: String ) -> String
{
    let mut outer_structure = "[{\n\n}]".to_string();
    let mut middle_structure = "\t\"\":".to_string();
    let mut final_inner_structure = "\n\t\t[{\"\": }]".to_string();
    middle_structure.insert_str(4, &final_inner_structure);//start with the final inner structure
                                                           //add a new one on every entry to
                                                           //the top
    outer_structure.insert_str(3 , &middle_structure);
    let mut json_block = outer_structure.clone();
    return json_block;
}

fn make_first_entry(mut json_block: String, mut title: String, mut data:  String, mut n: String) -> String
{
    let t_len = title.len();
    let d_len = data.len();
    let n_len = n.len();
    json_block.insert_str(5 ,&title);//only accounting for starting block aka outer structure
    json_block.insert_str(13+t_len ,&data);//accounting for title added in this step
    json_block.insert_str(16+d_len+t_len ,&n);//accounting for title and data here
    return json_block;
}

fn make_entry(mut json_block: String, mut title: String, mut data:  String, mut n: String) -> String
{
    let t_len = title.len();
    let d_len = data.len();
    let n_len = n.len();
    let mut new_inner_structure = "\n\t\t[{\"\": }],\n".to_string();
    let mut middle_structure = "\t\"\":".to_string();
    new_inner_structure.insert_str(6, &data);
    new_inner_structure.insert_str(9+d_len, &n);
    middle_structure.insert_str(2, &title);
    middle_structure.insert_str(4+t_len, &new_inner_structure);
    json_block.insert_str(3, &middle_structure);
    return json_block;    
}

fn update_entry(mut json_block: String, mut title: String, mut data:  String, mut n: String) -> String
{
    let t_len = title.len();
    let d_len = data.len();
    let n_len = n.len();
    return json_block;
}
