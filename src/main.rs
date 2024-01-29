use std::{thread};
use std::collections::HashMap;
use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use threadpool::ThreadPool;
use crate::core::server_cache;
use crate::config::Config;
use crate::content_manager::content_parser;
use crate::core::server_cache::FileCacheTuple;
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
    let pool = ThreadPool::new(conf.active_threads);
    server_cache::cache_init(conf);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        pool.execute(move|| {
            handle_http_req(&mut stream).expect("TODO: panic message");
        });
    }
}


fn handle_http_req(request: &mut TcpStream) -> Result<(), ()>{
    let resp: ReferaResponse;
    let user_agent_error = String::from("Unknown user agent");
    let binding = String::from(" ");
    let http_error = "Non-HTTP request";

    let http_headers = get_headers(&request);

    let headers: HashMap<String, String> = http_headers.0;
    let method_url: Vec<_> = http_headers.1.split(" ").collect(); //Returns ["HTTP-METHOD-NAME", "url", "HTTP/X.X"]

    let http_method = method_url.get(0).unwrap_or(&"Unknown method");
    let req_url = method_url.get(1).unwrap_or(&"");
    let http_version = method_url.get(2).unwrap_or(&http_error);
    let user_agent = headers.get("User-Agent").unwrap_or(&user_agent_error);
    let referer = headers.get("Referer").unwrap_or(&binding);
    let mut possible_ip = headers.get("X-Forwarded-For").unwrap_or(&binding);

    let _a = request.peer_addr().unwrap();
    let b = _a.ip().to_string();

    if possible_ip.eq(&binding){
        possible_ip = &b;
    }
    log_request(possible_ip, http_method, req_url,
                user_agent, referer);

    if !http_version.contains("HTTP/1.1") && !http_version.contains("HTTP/1.0") && !http_version.contains("HTTP/2.0")  {
        resp = ReferaResponse::new(StatusCode::BadRequest, None, Vec::new());
        &request.write_all(resp.as_u8().as_slice()).unwrap();
        return Ok(());
    }

    if http_method.contains("GET") {
        resp = get_reply(req_url, headers);
    } else {
        resp = ReferaResponse::new(StatusCode::NoContent, None, Vec::new())
    }
    &request.write_all(resp.as_u8().as_slice()).unwrap();
    Ok(())
}

fn get_reply(url: &str, request_headers: HashMap<String, String>) -> ReferaResponse {
    let size: usize;
    let no_query_params = &url[0..url.find("?").unwrap_or(url.len())];

    let cached_file = server_cache::file_lookup(no_query_params);
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Server".parse().unwrap(), "refera-RustWebServer".parse().unwrap());
    headers.insert("originating-server".parse().unwrap(), "refera-RustWebServer".parse().unwrap());


    if cached_file.is_none() {
        let file = content_parser::get_file(no_query_params);
        return if !file.0.is_empty() {
            server_cache::insert_file(no_query_params, &file);
            size = file.0.len();

            headers.insert("Content-Length".parse().unwrap(), size.to_string());
            ReferaResponse::new(StatusCode::Ok, Option::from(headers), file.0)
        } else {
            ReferaResponse::new(StatusCode::NotFound, None, content_parser::error_page())
        };
    }
    let cf  = cached_file.unwrap();
    size = cf.2;
    headers.insert("Content-Length".parse().unwrap(), size.to_string());
    ReferaResponse::new(StatusCode::Ok, Option::from(headers), cf.0)
}


