use std::{env, fs, thread};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use crate::config::Config;
use crate::response::ReferaResponse;

mod config;
mod response;
mod content_parser;
mod error;

fn main() {
    let mut conf = Config::default_config();
    let args = env::args();
    if args.len() < 1 {
        //TODO: Replace default config values with the args, if given.
    }
    let address = conf.address.to_string().add(":").add(&*conf.port);

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            // read( &mut  stream);
            handle_http_req(&mut stream);
        });
    }
    //Ok(())
}


fn handle_http_req( mut request: &mut TcpStream) {
  //  let mut request_reader = BufReader::new(request);
   // request_reader.


    let mut buf_reader = BufReader::new( request.try_clone().unwrap());


    let header_vector: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

   let http_headers = determine_headers(&header_vector);

    let method_url: Vec<_> = http_headers.1.split(" ").collect(); //Returns ["HTTP-METHOD-NAME", "url", "HTTP/X.X"]

    let resp: ReferaResponse;

    if method_url.get(0).unwrap().contains("GET") {
        resp = get_reply(method_url.get(1).unwrap(), http_headers.0);
    } else if method_url.get(0).unwrap().contains("POST"){
        resp = post_reply(&mut buf_reader,  http_headers.0);
    }
    else{
        resp =   ReferaResponse::new("HTTP/1.1 200 OK".parse().unwrap(), None, Vec::new())
    }
    &request.write_all(resp.as_u8().as_slice()).unwrap();


}

fn get_reply(url: &str, headers: HashMap<String, String>) -> ReferaResponse {
    let html = content_parser::get_content(url);

    ReferaResponse::new("HTTP/1.1 200 OK".parse().unwrap(), None, html)
}



fn post_reply(buf_reader: &mut BufReader<TcpStream>, headers: HashMap<String, String>) -> ReferaResponse{  //WIP - TODO
    let content_length_str = headers.get_key_value("Content-Length").unwrap().1;
    let mut buffer: Vec<u8> = vec![0; content_length_str.trim().parse::<usize>().unwrap()];
    buf_reader.read_exact(&mut buffer).unwrap();
    let result = content_parser::post_content(buffer.clone(), "aa");
    println!("wrote in file");


    ReferaResponse::new("HTTP/1.1 200 OK".parse().unwrap(), None, Vec::new())

}

fn determine_headers(vector: &Vec<String>) -> (HashMap<String, String>, String){
    let mut header_map = HashMap::new();

    let request_type = vector.get(0).unwrap().clone();

    for i in 1..vector.len(){
        let splittable = vector.get(i).unwrap();
        let mut splitted: Vec<String> = splittable.split(":").map(|s| s.to_string()).collect();
        let name = String::from(( splitted).get(0).unwrap());
        splitted.remove(0);
        let value = (splitted).concat();
        header_map.insert(name, value);
    }
    (header_map, request_type)
}


fn post_reply_alt(request: &mut TcpStream) -> ReferaResponse {  //WIP - TODO
    let mut buffer = vec![0; 1024];
    let bb = request.read_to_end(&mut buffer).unwrap();
    // println!("{}", String::from_utf8(content_vector.clone()).unwrap());
    println!("{bb}");
    ReferaResponse::new("HTTP/1.1 200 OK".parse().unwrap(), None, Vec::new())
}

fn read(stream: &mut TcpStream) {
    let mut buf = vec![0; 1024];
    println!("Received {} bytes", stream.read(&mut buf).unwrap());
    let resp = ReferaResponse::new("HTTP/1.1 200 OK".parse().unwrap(), None, Vec::new());
    stream.write_all(resp.as_u8().as_slice()).unwrap();
}




/*
    This causes the request to stack. Why?
    According to stackoverflow, "read_to_string reads into String until EOF which will not happen until you close the stream from the writer side."

fn experiment(mut req: TcpStream) {
    let mut str = String::new();

    &req.read_to_string(&mut str).unwrap();
    let linesplit: Vec<&str> = str.split("\n").collect();

    let head = linesplit.get(0).unwrap();
    if !head.contains("HTTP/1.1")   ||  !head.contains("HTTP/2"){
        //TODO: quinn gia http 3, kai quic protocol
    }

    if head.contains("GET") {
        let resp = get_reply(&req);
        //println!(" {}", String::from_utf8(resp.body.clone()).unwrap());

        //req.write_all(&*resp.as_u8()).unwrap();
        req.write_all(resp.as_str().as_bytes()).unwrap();
        println!(" written");
    }
}*/