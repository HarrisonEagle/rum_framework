pub const CONTINUE: i32 = 100;
pub const SWITCHING_PROTOCOLS: i32 = 101;
pub const OK: i32 = 200;
pub const CREATED: i32 = 201;
pub const ACCEPTED: i32 = 202;
pub const NON_AUTHORITATIVE_INFORMATION: i32 = 203;
pub const NO_CONTENT: i32 = 204;
pub const RESET_CONTENT: i32 = 205;
pub const PARTIAL_CONTENT: i32 = 206;
pub const MULTI_STATUS: i32 = 207;
pub const ALREADY_REPORTED: i32 = 208;
pub const IM_USED: i32 = 226;
pub const MULTIPLE_CHOICES: i32 = 300;
pub const MOVED_PERMANENTLY: i32 = 301;
pub const FOUND: i32 = 302;
pub const SEE_OTHER: i32 = 303;
pub const NOT_MODIFIED: i32 = 304;
pub const USE_PROXY: i32 = 305;
pub const TEMPORARY_REDIRECT: i32 = 307;
pub const PERMANENT_REDIRECT: i32 = 308;
pub const BAD_REQUEST: i32 = 400;
pub const UNAUTHORIZED: i32 = 401;
pub const PAYMENT_REQUIRED: i32 = 402;
pub const FORBIDDEN: i32 = 403;
pub const NOT_FOUND: i32 = 404;
pub const METHOD_NOT_ALLOWED: i32 = 405;
pub const NOT_ACCEPTABLE: i32 = 406;
pub const PROXY_AUTHENTICATION_REQUIRED: i32 = 407;
pub const REQUEST_TIMEOUT: i32 = 408;
pub const CONFLICT: i32 = 409;
pub const GONE: i32 = 410;
pub const LENGTH_REQUIRED: i32 = 411;
pub const PRECONDITION_FAILED: i32 = 412;
pub const PAYLOAD_TOO_LARGE: i32 = 413;
pub const URI_TOO_LONG: i32 = 414;
pub const UNSUPPORTED_MEDIA_TYPE: i32 = 415;
pub const RANGE_NOT_SATISFIABLE: i32 = 416;
pub const EXPECTATION_FAILED: i32 = 417;
pub const IM_A_TEAPOT: i32 = 418;
pub const MISDIRECTED_REQUEST: i32 = 421;
pub const UNPROCESSABLE_ENTITY: i32 = 422;
pub const LOCKED: i32 = 423;
pub const FAILED_DEPENDENCY: i32 = 424;
pub const UPGRADE_REQUIRED: i32 = 426;
pub const PRECONDITION_REQUIRED: i32 = 428;
pub const TOO_MANY_REQUESTS: i32 = 429;
pub const REQUEST_HEADER_FIELDS_TOO_LARGE: i32 = 431;
pub const UNAVAILABLE_FOR_LEGAL_REASONS: i32 = 451;
pub const INTERNAL_SERVER_ERROR: i32 = 500;
pub const NOT_IMPLEMENTED: i32 = 501;
pub const BAD_GATEWAY: i32 = 502;
pub const SERVICE_UNAVAILABLE: i32 = 503;
pub const GATEWAY_TIMEOUT: i32 = 504;
pub const HTTP_VERSION_NOT_SUPPORTED: i32 = 505;
pub const VARIANT_ALSO_NEGOTIATES: i32 = 506;
pub const INSUFFICIENT_STORAGE: i32 = 507;
pub const LOOP_DETECTED: i32 = 508;
pub const NOT_EXTENDED: i32 = 510;
pub const NETWORK_AUTHENTICATION_REQUIRED: i32 = 511;

const STATUSES: [(i32, &str); 58] = [
    (CONTINUE,"100 Continue"),
    (SWITCHING_PROTOCOLS,"101 Switching Protocols"),
    (OK,"200 OK"),
    (CREATED,"201 Created"),
    (ACCEPTED,"202 Accepted"),
    (NON_AUTHORITATIVE_INFORMATION,"203 Non-Authoritative Information"),
    (NO_CONTENT,"204 No Content"),
    (RESET_CONTENT, "205 Reset Content"),
    (PARTIAL_CONTENT, "206 Partial Content"),
    (MULTI_STATUS, "207 Multi-Status"),
    (ALREADY_REPORTED, "208 Already Reported"),
    (IM_USED, "226 IM Used"),
    (MULTIPLE_CHOICES, "300 Multiple Choices"),
    (MOVED_PERMANENTLY, "301 Moved Permanently"),
    (FOUND, "302 Found"),
    (SEE_OTHER, "303 See Other"),
    (NOT_MODIFIED, "305 Not Modified"),
    (TEMPORARY_REDIRECT, "307 Temporary Redirect"),
    (PERMANENT_REDIRECT, "308 Permanent Redirect"),
    (BAD_REQUEST, "400 Bad Request"),
    (UNAUTHORIZED, "401 Unauthorized"),
    (PAYMENT_REQUIRED, "402 Payment Required"),
    (FORBIDDEN, "403 Forbidden"),
    (NOT_FOUND, "404 Not Found"),
    (METHOD_NOT_ALLOWED, "405 Method Not Allowed"),
    (NOT_ACCEPTABLE, "406 Not Acceptable"),
    (PROXY_AUTHENTICATION_REQUIRED, "407 Proxy Authentication Required"),
    (REQUEST_TIMEOUT, "408 Request Timeout"),
    (CONFLICT, "409 Conflict"),
    (GONE, "410 Gone"),
    (LENGTH_REQUIRED, "411 Length Required"),
    (PRECONDITION_FAILED, "412 Precondition Failed"),
    (PAYLOAD_TOO_LARGE, "413 Payload Too Large"),
    (URI_TOO_LONG, "414 URI Too Long"),
    (UNSUPPORTED_MEDIA_TYPE, "415 Unsupported Media Type"),
    (RANGE_NOT_SATISFIABLE, "416 Range Not Satisfiable"),
    (EXPECTATION_FAILED, "417 Expectation Failed"),
    (IM_A_TEAPOT, "418 I'm a teapot"),
    (MISDIRECTED_REQUEST, "421 Misdirected Request"),
    (UNPROCESSABLE_ENTITY, "422 Unprocessable Entity"),
    (LOCKED, "423 Locked"),
    (FAILED_DEPENDENCY, "424 Failed Dependency"),
    (UPGRADE_REQUIRED, "426 Upgrade Required"),
    (PRECONDITION_REQUIRED, "428 Precondition Required"),
    (TOO_MANY_REQUESTS, "429 Too Many Requests"),
    (REQUEST_HEADER_FIELDS_TOO_LARGE, "431 Request Header Fields Too Large"),
    (UNAVAILABLE_FOR_LEGAL_REASONS, "451 Unavailable For Legal Reasons"),
    (INTERNAL_SERVER_ERROR, "500 Internal Server Error"),
    (NOT_IMPLEMENTED, "501 Not Implemented"),
    (BAD_GATEWAY, "502 Bad Gateway"),
    (SERVICE_UNAVAILABLE, "503 Service Unavailable"),
    (GATEWAY_TIMEOUT, "504 Gateway Timeout"),
    (HTTP_VERSION_NOT_SUPPORTED, "505 HTTP Version Not Supported"),
    (VARIANT_ALSO_NEGOTIATES, "506 Variant Also Negotiates"),
    (INSUFFICIENT_STORAGE, "507 Insufficient Storage"),
    (LOOP_DETECTED, "508 Loop Detected"),
    (NOT_EXTENDED, "510 Not Extended"),
    (NETWORK_AUTHENTICATION_REQUIRED, "511 Network Authentication Required")
];

pub fn from_status_code(status_code: i32) -> String{
    for status in STATUSES {
        if status_code == status.0{
            return status.1.to_string();
        } 
    }
    return "".to_string();
}