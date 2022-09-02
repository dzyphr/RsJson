use serde_json::Value;
use crate::Logic;
use crate::Logic::{ True, False };
use crate::LinearFn;
use crate::LinearFn::{ MakeEntry, UpdateEntry };
use crate::{ Json_Structures, Operations, make_entry, update_entry };
use std::{collections::{ HashMap } };
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
            match qualified_function.clone()
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

