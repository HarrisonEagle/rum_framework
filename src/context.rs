use std::{str::from_utf8_unchecked, io::{BufReader, Read}, fs::File, collections::BTreeMap};

use tera::{Context, Tera};

use crate::{ status_code::{self, HTTP_VERSION_NOT_SUPPORTED, INTERNAL_SERVER_ERROR, NOT_FOUND, from_status_code, BAD_REQUEST}, response::{Response, self}};

pub struct RumContext<'a>{
    template_engine: Option<&'a Tera>,
    request_header: BTreeMap<String, String>,
    request_body: String,
    response_header: BTreeMap<String, String>,
    response: Option<Response>,
}

impl RumContext<'_> {

    pub fn new(template_engine: Option<&Tera>) -> RumContext{
        return RumContext{
            template_engine: template_engine,
            request_header: BTreeMap::default(),
            request_body: "".to_string(),
            response_header: BTreeMap::default(),
            response: None,
        };
    }
    
    pub fn html(&mut self, status_code: i32, template_name: &str, context: &Context) {
        if self.template_engine.is_some(){
            let template = self.template_engine.unwrap();
            return match template.render(template_name, context){
                Ok(html) => {
                    self.set_response_header("Content-Type", &(mime::TEXT_HTML_UTF_8.to_string()));
                    self.response = Some(Response {
                        http_status: status_code::from_status_code(status_code),
                        response_body: html,
                    })
                    
                },
                Err(e) => {
                    self.set_response_header("Content-Type", &(mime::TEXT_HTML_UTF_8.to_string()));
                    self.response = Some(Response {
                        http_status: status_code::from_status_code(status_code::INTERNAL_SERVER_ERROR),
                        response_body: format!("{}\n",e),
                    })
                },
            }
        }
        self.set_response_header("Content-Type", &(mime::TEXT_HTML_UTF_8.to_string()));
        self.response = Some(Response {
            http_status: status_code::from_status_code(status_code::INTERNAL_SERVER_ERROR),
            response_body: "Tera template engine not initialized properly".to_string(),
        })
    }

    pub fn text(&mut self, status_code: i32, text: &str){
        self.set_response_header("Content-Type", &mime::TEXT.as_str());
        self.response = Some(Response {
            http_status: status_code::from_status_code(status_code),
            response_body: text.to_string(),
        })
    }

    pub fn json(&mut self, status_code: i32, json_str: String){
        self.set_response_header("Content-Type", &(mime::APPLICATION_JSON.to_string()));
        self.response = Some( Response {
            http_status: status_code::from_status_code(status_code),
            response_body: json_str.to_string(),
        })
    }

    pub fn file(&mut self, status_code: i32, file_path: &str){
        return match File::open(file_path){
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                // Read file into vector.
                match reader.read_to_end(&mut buffer){
                    Ok(_) => {
                        self.set_response_header("Content-Type", &(match mime_guess::from_path(file_path).first(){
                            Some(mime) => { mime.to_string() },
                            None => { mime::TEXT.to_string() },
                        }));
                        self.response = Some(Response {
                            http_status: status_code::from_status_code(status_code),
                            response_body: unsafe {from_utf8_unchecked(&buffer).to_string() },
                        })
                    },

                    Err(e) => {
                        self.set_response_header("Content-Type", &(mime::TEXT_HTML_UTF_8.to_string()));
                        self.response = Some(Response {
                            http_status: status_code::from_status_code(status_code::INTERNAL_SERVER_ERROR),
                            response_body: e.to_string(),
                        })
                    }
                }
                
            },
            Err(_) => {
                self.set_response_header("Content-Type", &(mime::TEXT_HTML_UTF_8.to_string()));
                self.response = Some(Response {
                    http_status: status_code::from_status_code(status_code::NOT_FOUND),
                    response_body: "Not Found".to_string(),
                })
            },
        };
    }

    pub(crate) fn set_request_header(&mut self, key: &str, value: &str){
        self.request_header.insert(key.to_string(), value.to_string());
    }

    pub(crate) fn set_request_body(&mut self, request_body: String){
        self.request_body = request_body;
    }

    pub fn get_request_header(&self, key: &str) -> Option<&String>{
        return self.request_header.get(key);
    }

    pub fn set_response_header(&mut self, key: &str, value: &str){
        self.response_header.insert(key.to_string(), value.to_string());
    }

    pub fn remove_response_header(&mut self, key: &str){
        self.response_header.remove(key);
    }

    pub(crate) fn get_response_headers(&self){
        let mut response_headers = String::new();
        
    }

    pub(crate) fn get_response(&self, http_ver: &str) -> (String, String){
        return match &self.response{
            Some(response) => {
                response.get_response(http_ver)
            },
            None => {
                let status = from_status_code(INTERNAL_SERVER_ERROR);
                (format!("{} {}\r\n{}\r\n{}", http_ver, status, "Content-Type: text\r\n", "Error:Response Not Found!"), status)
            }
        }
    }

    pub(crate) fn default_404(&self, http_ver: &str) -> (String, String){
        let status = from_status_code(NOT_FOUND);
        return (format!("{} {}\r\n{}\r\n{}", http_ver, status, "Content-Type: text\r\n", "Error:Resource Not Found!"), status)
    }

    pub(crate) fn default_400(&self, http_ver: &str) -> (String, String){
        let status = from_status_code(BAD_REQUEST);
        return (format!("{} {}\r\n{}\r\n{}", http_ver, status, "Content-Type: text\r\n", "Error:Method Not Found!"), status)
    }
}