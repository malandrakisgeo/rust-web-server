use std::fs;
use std::fs::ReadDir;
use std::os::unix::fs::MetadataExt;
use chrono::{DateTime, Utc};

/*
    The reason this project exists. I wanted to improve my Rust skills by writing a server that renders an HTML file with a list of songs,
    and serves them in demand. But I chose to make it a general purpose web server instead.
    TODO: Create a separate branch with the song_site
 */

pub fn songs_page(mut contents: Vec<u8>) -> Vec<u8> {
    let paths = fs::read_dir("./static/malandrakisgeo-songs-site-example/songs/").unwrap_or(fs::read_dir(".").unwrap());
    let tds = generate_tds(paths);

    let str = String::from_utf8(contents).unwrap();
    let content_to_serve = str.replace("{replace_me!}", &tds);

    Vec::from(content_to_serve)
}
fn generate_tds(files: ReadDir) -> String {
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

