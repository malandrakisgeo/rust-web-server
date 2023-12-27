use std::{fs};
use std::fs::{ReadDir};
use std::io::Write;
//use std::io::{Read};
use std::os::unix::fs::MetadataExt;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
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


fn songs_page(mut contents: Vec<u8>) -> Vec<u8> {
    let paths = fs::read_dir("./static/malandrakisgeo-songs-site-example/songs/").unwrap_or(fs::read_dir(".").unwrap());
    let tds = generate_tds(paths);

    let str = String::from_utf8(contents).unwrap();
    let content_to_serve = str.replace("{replace_me!}", &tds);

    Vec::from(content_to_serve)
}


pub fn error_page() -> Vec<u8> {
    let error_page = fs::read_to_string("./static/not_found.html").unwrap();

    Vec::from(error_page)
}


fn parse_file(path: &str) -> Result<Vec<u8>, ReferaError> {
    // let proper_path = path.replace("%20", " "); //TODO: Fix
    let mut file: Vec<u8> = Vec::new();
    let decoded = decode(path).expect("UTF-8");

    if fs::read_dir("./static".to_owned() + path).is_err() {
        file = fs::read("./static".to_owned() + decoded.as_ref()).map_err(|e| ReferaError::from(e))?;
    } else { //if the url corresponds to a directory under /static
        file = fs::read("./static".to_owned() + decoded.as_ref() + "/index.html").map_err(|e| ReferaError::from(e))?; //retrieve the index.html
    }

    if (path.contains("malandrakisgeo") && !path.contains(".mp3")) {
        file = songs_page(file);
    }

    Ok(file)
}


/*fn parse_song_evolved(path: &str)-> Result<Vec<u8>, ReferaError>{
    let decoded = decode(path).expect("UTF-8");

    const BUFFER_LEN: usize = 51200;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open("./static/songs".to_owned()+decoded.as_ref())?;
    file.read(&mut buffer).unwrap();

    Ok(Vec::from(buffer))

}*/

pub fn generate_tds(files: ReadDir) -> String {
    let mut tds_vec = Vec::new();

    for file in files {
        let data = &file.unwrap();
        let path = &data.path();
        let _name = data.file_name();
        let file_name = _name.to_str().unwrap();

        let date_modified = data.metadata().unwrap().modified().unwrap(); //TODO: fix
        let dt: DateTime<Utc> = date_modified.clone().into();
        let showable_datetime = dt.format("%d/%m/%Y"); //%d/%m/%Y %T to show even time

        let file_type = std::path::Path::new(path.as_path()).extension().unwrap_or("unknown_type".as_ref()).to_str().unwrap();
        let size = &data.metadata().unwrap().size();

        let td = format!("<tr>\n<td><a href=\"songs/{file_name}\">{file_name}</a> </td>\n<td>{showable_datetime}</td>\n<td>{file_type}</td>\n<td>{size}</td>\n</tr>\n");
        //  let td = format!("<tr>\n<td><a href=\"malandrakisgeo-songs-site-example/songs/{file_name}\">{file_name}</a> </td>\n<td>{showable_datetime}</td>\n<td>{file_type}</td>\n<td>{size}</td>\n</tr>\n");

        tds_vec.append(&mut Vec::from(td));
    }

    String::from_utf8(tds_vec).unwrap()
}

