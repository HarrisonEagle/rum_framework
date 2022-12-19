use std::{str::from_utf8_unchecked, io::{BufReader, Read}, fs::File};

use tera::{Context, Tera};

use crate::{router::{Response, ResponseType}, status_code};

pub struct RumContext<'a>{
    template_engine: Option<&'a Tera>
}

impl RumContext<'_> {

    pub fn new(template_engine: Option<&Tera>) -> RumContext{
        return RumContext{
            template_engine: template_engine,
        };
    }
    
    pub fn html(&self, status_code: i32, template_name: &str, context: &Context) -> Response{
        if self.template_engine.is_some(){
            let template = self.template_engine.unwrap();
            return match template.render(template_name, context){
                Ok(html) => {
                    Response {
                        http_status: status_code::from_status_code(status_code),
                        content_type: mime::HTML.to_string(),
                        response_type: ResponseType::Html,
                        response_body: html,
                    }
                },
                Err(e) => {
                    Response {
                        http_status: status_code::from_status_code(status_code::INTERNAL_SERVER_ERROR),
                        content_type: mime::HTML.to_string(),
                        response_type: ResponseType::Html,
                        response_body: format!("{}\n",e),
                    }
                },
            }
        }
        return Response {
            http_status: status_code::from_status_code(status_code::INTERNAL_SERVER_ERROR),
            content_type: mime::HTML.to_string(),
            response_type: ResponseType::Html,
            response_body: "Tera template engine not initialized properly".to_string(),
        }
    }

    pub fn file(&self, status_code: i32, file_path: &str) -> Response{
        // Read file into vector.
        println!("Searching:{}", file_path);
        return match File::open(file_path){
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                // Read file into vector.
                match reader.read_to_end(&mut buffer){
                    Ok(_) => {
                        let body = unsafe {from_utf8_unchecked(&buffer).to_string() };
                        Response {
                            http_status: status_code::from_status_code(status_code),
                            content_type: match mime_guess::from_path(file_path).first(){
                                Some(mime) => { mime.to_string() },
                                None => { mime::TEXT.to_string() },
                            },
                            response_type: ResponseType::Html,
                            response_body: body,
                        }
                    },

                    Err(e) => {
                        Response {
                            http_status: status_code::from_status_code(status_code::INTERNAL_SERVER_ERROR),
                            content_type: mime::HTML.to_string(),
                            response_type: ResponseType::Html,
                            response_body: e.to_string(),
                        }
                    }
                }
                
            },
            Err(_) => {
                Response {
                    http_status: status_code::from_status_code(status_code::NOT_FOUND),
                    content_type: mime::HTML.to_string(),
                    response_type: ResponseType::Html,
                    response_body: "Not Found".to_string(),
                }
            },
        };
    }
}