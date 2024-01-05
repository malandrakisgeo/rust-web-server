use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

pub fn get_headers(request: &TcpStream) -> (HashMap<String, String>, String)  {
    let mut buf_reader = BufReader::new(request);

    let header_vector: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    determine_headers(&header_vector)

}

fn determine_headers(vector: &Vec<String>) -> (HashMap<String, String>, String) {
    let mut header_map = HashMap::new();

    let request_type = String::from(vector.get(0).unwrap_or(&String::from("")));

    for i in 1..vector.len() {
        let splittable = vector.get(i).unwrap();
        let mut splitted: Vec<String> = splittable.split(":").map(|s| s.to_string()).collect();
        let name = String::from((splitted).get(0).unwrap());
        splitted.remove(0);
        let value = (splitted).concat();
        header_map.insert(name, value);
    }
    (header_map, request_type)
}
