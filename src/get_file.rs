use std::{fs::{File, }, io::{Read}, path::Path};
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

