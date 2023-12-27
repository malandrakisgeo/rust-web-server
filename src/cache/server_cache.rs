use std::collections::HashMap;
use std::thread;
use std::time::SystemTime;
use crate::config::Config;

#[derive(Clone)]
pub struct FileCacheTuple(pub Vec<u8>, pub SystemTime, pub usize);

static mut FILE_CACHE: Option<HashMap<String, FileCacheTuple>> = None;
static mut MAX_CACHE_FILES: usize = 0;
static mut MAX_FILE_SIZE: usize = 0;

pub fn cache_init(config: Config) {
    unsafe {
        FILE_CACHE = Some(HashMap::new());
        MAX_CACHE_FILES = config.max_cache_files;
        MAX_FILE_SIZE = config.largest_cacheable_file_size
    }
    thread::spawn(move || {
        loop {
            check_and_clean();
        }
    });
}

pub fn file_lookup(name: &str) -> Option<FileCacheTuple> {
    let mut file: Option<FileCacheTuple> = Option::None;
    unsafe {
        if (&FILE_CACHE.as_ref()).unwrap().get(name).is_some() {
            file = (&FILE_CACHE.as_ref()).unwrap().get(name).cloned();
        }
    }

    file
}

pub fn insert_file(name: &str, file: &FileCacheTuple) {
    unsafe {
        if file.0.len() < MAX_FILE_SIZE{
            let _ = &FILE_CACHE.as_mut().unwrap().insert(name.parse().unwrap(), file.clone());
        }
    }
}


fn remove_oldest() {
    let mut name: &str = " ";
    let mut oldest: SystemTime = SystemTime::now();
    unsafe {
        for (key, val) in FILE_CACHE.as_mut().unwrap().iter() {
            if oldest.min(val.1) != oldest { //if older than the oldest
                oldest = val.1;
                name = key;
            }
        }
        //    println!("{}", oldest);
        let _ = &FILE_CACHE.as_mut().unwrap().remove(name).unwrap();
    }
}


fn check_and_clean() {
    unsafe {
        if &FILE_CACHE.as_mut().unwrap().len() > &MAX_CACHE_FILES { //Add more songs under /static/songs and test it
            remove_oldest(); //TODO: Add DeepSize
        }
    }
}