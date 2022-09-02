pub fn make_first_entry(mut json_block: String, key: String, data:  String, n: String) -> String
{
    let t_len = key.len();
    let d_len = data.len();
    json_block.insert_str(5 ,&key);//only accounting for starting block aka outer structure
    json_block.insert_str(13+t_len ,&data);//accounting for key added in this step
    json_block.insert_str(16+d_len+t_len ,&n);//accounting for key and data here
    return json_block;
}

