
pub struct Response{
    pub(crate) http_status: String,
    pub(crate) response_body: String
}

impl Response {
    pub(crate) fn get_response(&self, http_ver: &str) -> (String, String){
        return (format!("{} {}\r\n{}\r\n{}", http_ver, self.http_status, "\r\n", self.response_body), self.http_status.clone());
    }
}