use std::collections::HashMap;
use std::thread;
use std::time::SystemTime;

#[derive(Clone)]
pub struct FileCacheTuple(pub Vec<u8>, pub SystemTime, pub usize);

static mut FILE_CACHE: Option<HashMap<String, FileCacheTuple>> = None;

pub fn cache_init() {
    unsafe { FILE_CACHE = Some(HashMap::new()); }
    thread::spawn(move || {
        while true { //TODO: Perhaps find some more elegant way?
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
        &FILE_CACHE.as_mut().unwrap().insert(name.parse().unwrap(), file.clone());
    }
}


fn remove_oldest() {
    let mut name: &str = " ";
    let mut oldest: SystemTime = SystemTime::now();
    unsafe {
        for (key, val) in FILE_CACHE.as_mut().unwrap().iter() {
            if (oldest.min(val.1) != oldest) { //if the oldest is not the oldest
                oldest = val.1;
                name = key;
            }
        }
        //    println!("{}", oldest);

        &FILE_CACHE.as_mut().unwrap().remove(name).unwrap();
    }
}


fn check_and_clean() {
    unsafe {
        if (&FILE_CACHE.as_mut().unwrap().len() > &3) { //Add more songs under /static/songs and test it
            remove_oldest(); //TODO: Add DeepSize
        }
    }
}