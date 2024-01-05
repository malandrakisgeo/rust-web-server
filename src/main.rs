use std::{thread};
use std::collections::HashMap;
use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use crate::core::server_cache;
use crate::config::Config;
use crate::content_manager::content_parser;
use crate::http::http_status::StatusCode;
use crate::http::util::get_headers;
use crate::logger::refera_logger::{flush_log, log_request};
use crate::response::refera_response::ReferaResponse;

mod config;
mod response;
mod http;
mod core;
mod content_manager;
mod logger;

fn main() {
    let conf = Config::default_config();

    let address = conf.address.to_string().add(":").add(&*conf.port);
    ctrlc::set_handler(move || flush_log())
        .expect("Error setting Ctrl-C handler");
    let listener = TcpListener::bind(address).unwrap();

    server_cache::cache_init(conf);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            handle_http_req(&mut stream);
        });
    }
}


fn handle_http_req(request: &mut TcpStream) {
    let resp: ReferaResponse;
    let user_agent_error = String::from("Unknown user agent");
    let http_error = "Non-HTTP request";

    let http_headers = get_headers(&request);

    let headers: HashMap<String, String> = http_headers.0;
    let method_url: Vec<_> = http_headers.1.split(" ").collect(); //Returns ["HTTP-METHOD-NAME", "url", "HTTP/X.X"]

    let http_method = method_url.get(0).unwrap_or(&"Unknown method");
    let req_url = method_url.get(1).unwrap_or(&"");
    let http_version = method_url.get(2).unwrap_or(&http_error);
    let user_agent = headers.get("User-Agent").unwrap_or(&user_agent_error);

    log_request(request.peer_addr().unwrap(), http_method, req_url,
                user_agent);

    if !http_version.contains("HTTP/1.1") && !http_version.contains("HTTP/1.0") {
        resp = ReferaResponse::new(StatusCode::BadRequest, None, Vec::new());
        &request.write_all(resp.as_u8().as_slice()).unwrap();
        return;
    }

    if http_method.contains("GET") {
        resp = get_reply(req_url, headers);
    } else {
        resp = ReferaResponse::new(StatusCode::BadRequest, None, Vec::new())
    }
    &request.write_all(resp.as_u8().as_slice()).unwrap();
}

fn get_reply(url: &str, headers: HashMap<String, String>) -> ReferaResponse {
    let cached_file = server_cache::file_lookup(url);
    if cached_file.is_none() {
        let file = content_parser::get_file(url);
        return if !file.0.is_empty() {
            server_cache::insert_file(url, &file);
            ReferaResponse::new(StatusCode::Ok, None, file.0.clone())
        } else {
            ReferaResponse::new(StatusCode::NotFound, None, content_parser::error_page())
        };
    }

    ReferaResponse::new(StatusCode::Ok, None, cached_file.unwrap().0.clone())
}


