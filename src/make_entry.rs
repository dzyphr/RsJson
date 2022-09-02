use crate::Json_Structures;
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
    if top_down == true
    {
        middle_structure.insert_str(4, &key);//shifted both up by two compared to bottom up json
        middle_structure.insert_str(6+t_len, &new_inner_structure);
    }
    else
    {
        middle_structure.insert_str(2, &key);
        middle_structure.insert_str(4+t_len, &new_inner_structure);
    }
    if top_down == false
    {
        json_block.insert_str(3, &middle_structure);
    }
    else
    {
        let entrypoint = json_block.len() - 3;
        json_block.insert_str(entrypoint, &middle_structure)//inserting in the length - 3 gives the next spot for top down
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

