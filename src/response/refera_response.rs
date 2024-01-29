use std::collections::HashMap;
use crate::http::http_status::StatusCode;


pub struct ReferaResponse {
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}


impl ReferaResponse {

    pub fn new(status: StatusCode, headers: Option<HashMap<String, String>>, body: Vec<u8>) -> ReferaResponse {
        return ReferaResponse {
            status: status.as_u16().to_string(),
            headers: headers.unwrap_or_else(|| HashMap::new()),
            body,
        };
    }

    pub fn as_u8(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.append(&mut Vec::from("HTTP/1.1 "));
        let status_str = self.status.clone() + "\r\n";
        let  headerss: Vec<String>  = self.headers.clone().into_iter()
            .map(|(header, value)| header + ": " + &value + "\r\n")
            .collect();

        let mut u8vec: Vec<u8>= headerss.clone().into_iter()
            .flat_map(|value| value.as_bytes().to_vec())
            .collect();

        //vec.append(&mut headers);
        //let content =
       //     format!("{status_str}\r\nContent-Length: {ln}\r\n\r\n");
       // vec.append(&mut Vec::from(content));
        vec.append(&mut Vec::from(status_str));
        vec.append(&mut u8vec);
        vec.append(&mut Vec::from( "\r\n"));
        vec.append(&mut self.body.clone());

        return vec;
    }

    pub fn as_str(&self) -> String {
        let status = &self.status;
        //let bofy = &self.body;
        let st = String::from_utf8(self.body.clone()).unwrap();
        let ln = st.len();

        let response =
            format!("{status}\r\nContent-Length: {ln}\r\n\r\n{st}");
        return response;
    }
}