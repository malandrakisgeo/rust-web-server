use std::{fs};
use std::time::SystemTime;
use urlencoding::decode;
use crate::response::refera_error::ReferaError;
use crate::server_cache::FileCacheTuple;


pub fn get_file(name: &str) -> FileCacheTuple {
    //TODO: Add support for range requests

    let f: FileCacheTuple;
    let mut ve: Vec<u8>;
    if name.eq("") || name.eq(" ") || name.eq("/") {
        ve = default_page();
    } else {
        ve = match parse_file(name) {
            Ok(data) => data,
            Err(err) => Vec::new()
        };
    }

    f = FileCacheTuple(ve, SystemTime::now(), 0);

    return f;
}


fn default_page() -> Vec<u8> {
    let contents = fs::read_to_string("./static/index.html").unwrap_or("Page not found!".parse().unwrap());

    Vec::from(contents)
}


pub fn error_page() -> Vec<u8> {
    let error_page = fs::read_to_string("./static/not_found.html")
        .unwrap_or_else(|_| { String::from("Not found!")});

    Vec::from(error_page)
}


fn parse_file(path: &str) -> Result<Vec<u8>, ReferaError> {
     let mut proper_path = path.replace("%20", " ");


    let mut file: Vec<u8> = Vec::new();
    let decoded = decode(path).expect("UTF-8");

    if fs::read_dir("./static".to_owned() + proper_path.as_mut_str()).is_err() { //if not a subdirectory
        file = fs::read("./static".to_owned() + decoded.as_ref()).map_err(|e| ReferaError::from(e))?;
    } else { //if the url corresponds to a directory under /static
        file = fs::read("./static".to_owned() + decoded.as_ref() + "/index.html").map_err(|e| ReferaError::from(e))?; //retrieve the index.html
    }



    Ok(file)
}

