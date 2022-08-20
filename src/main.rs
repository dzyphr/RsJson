use serde::Deserialize;
use serde_json::{Result, Value};
use std::{path::Path, fs::{File, OpenOptions}, io::{Read, Write}, collections::{HashMap}, process::{exit}};
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
    let full_filename = filename.to_owned() + ext;
    let mut json_block  = get_file(full_filename.clone());
/*    json_block = init_json(json_block.to_string());
    json_block = make_first_entry(
        json_block.clone().to_string(), "title0".to_string(), "data".to_string(), "6".to_string()
    );
    json_block = make_entry(
        json_block.clone().to_string(), "title1".to_string(), "data".to_string(), "6".to_string()
    );*/
    update_entry(json_block.clone(), "title0".to_string(), "data".to_string(), "ADD".to_string(), 1);
   // overwrite_data(json_block, full_filename);
}

fn overwrite_data(json_block: String, full_filename: String)
{
    let mut  file = File::create(full_filename.clone()).expect("error opening json file");
    file.write_all(&json_block.as_bytes());
}

fn get_file(full_filename: String) -> String
{
    let mut filecontents = String::new();
    if Path::new(&full_filename).exists()
    {
        let mut file = File::open(&full_filename).expect("unable to open json file");
        file.read_to_string(&mut filecontents).expect("unable to read from json file");
    }
    else
    {
        let mut file = File::create(full_filename).expect("error creating new json file");
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

fn update_entry(mut json_block: String, mut title: String, mut data:  String, mut opt: String, mut opt_val: i64) -> (String)
{
    struct Operations
    {
        ADD: String
    }
    let ops = Operations{ ADD: "ADD".to_string()};
    let ADD = ops.ADD;
    let t_len = title.len();
    let d_len = data.len();
    let mut old_value: i64 = 0;
    let jObj: Vec<HashMap<String, Value>> = serde_json::from_str(&json_block).expect("error indexing into json object");
    let mut i = 0;
    let mut was_found = false;
    for (x, y) in  &jObj.clone()[0]
    {
        i = i + 1;
        if x == &title
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
    let mut new_val = 0;
    match opt
    {
         ADD => new_val = add_opt(old_value, opt_val),
    };
    fn add_opt(old_data: i64, opt_val: i64) -> i64
    {
        return old_data + opt_val;
    }
    let t_loc = json_block.find(&title).expect("cannot find Key");
    let mut middle_structure = "\t\"\":".to_string();
    let mut final_inner_structure = "\n\t\t[{\"\": }]".to_string();
    let mut compare_model = String::new();
    middle_structure.insert_str(2, &title);
    final_inner_structure.insert_str(6, &data);
    final_inner_structure.insert_str(9+d_len, &old_value.to_string());
    middle_structure.insert_str(middle_structure.len(), &final_inner_structure);
    compare_model = middle_structure;
    middle_structure = "\t\"\":".to_string();
    final_inner_structure = "\n\t\t[{\"\": }]".to_string();
    let mut new_model = String::new();
    middle_structure.insert_str(2, &title);
    final_inner_structure.insert_str(6, &data);
    final_inner_structure.insert_str(9+d_len, &new_val.to_string());
    middle_structure.insert_str(middle_structure.len(), &final_inner_structure);
    new_model = middle_structure;
    json_block = json_block.replace(&compare_model, &new_model);
    dbg!(&json_block);
    return (json_block)
}
