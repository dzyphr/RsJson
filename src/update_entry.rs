use crate::{ Json_Structures, Operations , Operations::{ADD, SUB, MTP, DIV}, Value, exit, HashMap};
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
//            exit(1);
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
