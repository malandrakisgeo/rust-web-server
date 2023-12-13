use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Clone)]
pub struct FileCacheTuple(pub Vec<u8>, pub SystemTime, pub usize);

static mut FILE_CACHE: Option<HashMap<String, FileCacheTuple>> = None;

pub fn cache_init() {
    unsafe { FILE_CACHE = Some(HashMap::new()); }
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