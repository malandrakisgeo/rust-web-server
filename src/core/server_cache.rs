use std::collections::HashMap;
use std::time::SystemTime;
use once_cell::sync::Lazy;
use crate::config::Config;
use crate::core::mutex::Mutex;

#[derive(Clone)]
pub struct FileCacheTuple(pub Vec<u8>, pub SystemTime, pub usize);

static mut FILE_CACHE: Lazy<Mutex<HashMap<String, FileCacheTuple>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });
//= Mutex::new(HashMap::new());
static mut CURRENT_CACHE_SIZE: usize = 0;

static mut MAX_CACHE_FILES: usize = 0;
static mut MAX_FILE_SIZE: usize = 0;

pub fn cache_init(config: Config) {
    unsafe {
        MAX_CACHE_FILES = config.max_cache_files;
        MAX_FILE_SIZE = config.largest_cacheable_file_size;
        if MAX_CACHE_FILES == 0 || MAX_FILE_SIZE == 0{
            return ;
        }
    }
}

pub fn file_lookup(name: &str) -> Option<FileCacheTuple> {
    let mut file: Option<FileCacheTuple> = Option::None;
    unsafe {
        let cache = FILE_CACHE.acquire();
        if cache.get(name).is_some() {
            file = cache.get(name).cloned();
        }
        FILE_CACHE.free();
    }

    file
}

pub fn insert_file(name: &str, file: &FileCacheTuple) {
    unsafe {
        if file.0.len() < MAX_FILE_SIZE {
            let cache = FILE_CACHE.acquire_mut();
            while CURRENT_CACHE_SIZE >= MAX_CACHE_FILES {
                remove_oldest(cache);
            }
            let _ = &cache.insert(name.parse().unwrap(), file.clone());
            CURRENT_CACHE_SIZE += 1;
            FILE_CACHE.free();
        }
    }
}


fn remove_oldest(cache: &mut HashMap<String, FileCacheTuple>) {
    let mut name: &str = " ";
    let mut oldest: SystemTime = SystemTime::now();

    unsafe {
        let cl = cache.clone();
        for (key, val) in cl.iter() { //TODO: The same using a FIFO structure instead
            if oldest.min(val.1) != oldest { //if older than the oldest
                oldest = val.1;
                name = key;
            }
        }
        let _ = cache.remove(name).unwrap();
        CURRENT_CACHE_SIZE -= 1;
    }
}


