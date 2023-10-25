use std::{fs, io};
use std::fs::ReadDir;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Utc};
use crate::error::ReferaError;
use urlencoding::decode;


pub fn get_content(name: &str) -> Vec<u8>{
    if name.eq("") || name.eq(" ") || name.eq("/") {
        return default_page();
    }

    if(name.eq("/favicon.ico")){
        return Vec::new();
    }

     match parse_song(name){
         Ok(T) => T,
         Err(E) => error_page()
     }
}

pub fn post_content(content: Vec<u8>, name: &str) -> &str{
    println!("in post content"); //Result<&str, ReferaError>
    let mut creation = fs::File::create("./static/posted_by_user/".to_owned()+name).expect("failure");
    let size_written = creation.write_all(content.as_slice()).expect("failure!");

    "ok"
}

fn default_page() -> Vec<u8>{
    let paths = fs::read_dir("./static/songs").unwrap_or(fs::read_dir(".").unwrap());

    let contents = fs::read_to_string("./static/default.html").unwrap_or("Page not found!".parse().unwrap());
    let tds = generate_tds(paths);
    let content_to_serve = contents.replace("{replace_me!}", &tds);


    Vec::from(content_to_serve)
}

fn error_page() -> Vec<u8>{
    let error_page = fs::read_to_string("./static/not_found.html").unwrap();

    Vec::from(error_page)
}


fn parse_song(path: &str)-> Result<Vec<u8>, ReferaError>{
    println!("{}", &path);
    let proper_path = path.replace("%20", " "); //TODO: Fix
    let decoded = decode(path).expect("UTF-8");

    let mp3 = fs::read("./static/songs".to_owned()+decoded.as_ref()).map_err(|e| ReferaError::from(e))?;

    Ok(mp3)

}

pub fn generate_tds(files: ReadDir) -> String{
    let mut tds_vec = Vec::new();

    for file in files {
        let data = &file.unwrap();
        let path = &data.path();
        let _name = data.file_name();
        let file_name = _name.to_str().unwrap();

        let date_modified = data.metadata().unwrap().modified().unwrap(); //TODO: fix
        let dt: DateTime<Utc> = date_modified.clone().into();
        let showable_datetime =  dt.format("%d/%m/%Y"); //%d/%m/%Y %T to show even time

        let file_type = std::path::Path::new(path.as_path()).extension().unwrap_or("unknown_type".as_ref()).to_str().unwrap();
        let size = &data.metadata().unwrap().size();

        //WORKS: let td =  format!("<tr>\n<td>  {file_name}</td>\n<td>{showable_datetime}</td>\n<td>{file_type}</td>\n<td>{size}</td>\n</tr>\n");
        let td =  format!("<tr>\n<td><a href=\"{file_name}\">{file_name}</a> </td>\n<td>{showable_datetime}</td>\n<td>{file_type}</td>\n<td>{size}</td>\n</tr>\n");

        tds_vec.append(&mut Vec::from(td));
    }

    String::from_utf8(tds_vec).unwrap()


}

