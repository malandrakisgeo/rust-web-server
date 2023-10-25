use std::collections::HashMap;


pub struct ReferaResponse {
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}


impl ReferaResponse {
    pub fn new(status: String, headers: Option<HashMap<String, String>>, body: Vec<u8>) -> ReferaResponse {
        return ReferaResponse {
            status,
            headers: headers.unwrap_or_else(|| HashMap::new()),
            body,
        };
    }

    pub fn as_u8(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        let mut status: Vec<u8> = self.status.as_bytes().to_vec();
        let ln = self.body.len();
        let statusstr= self.status.clone();
        let mut headers: Vec<String>  = self.headers.clone().into_iter()
            .map(|(header, value)| header + ":" + &value)
            .collect();

        //vec.append(&mut status);
        //vec.append(&mut headers);
        let response =
            format!("{statusstr}\r\nContent-Length: {ln}\r\n\r\n");

        vec.append(&mut Vec::from(response));
        vec.append(&mut self.body.clone());
        ///println!("{}",         String::from_utf8(vec.clone()).unwrap());

        return vec;
    }

    pub fn as_str(&self) -> String {
        let status = &self.status;
        let bofy = &self.body;
        let st = String::from_utf8(self.body.clone()).unwrap();
        let ln = st.len();

        let response =
            format!("{status}\r\nContent-Length: {ln}\r\n\r\n{st}");
        return response;
    }
}