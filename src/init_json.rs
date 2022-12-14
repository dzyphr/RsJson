use crate::Json_Structures;
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

