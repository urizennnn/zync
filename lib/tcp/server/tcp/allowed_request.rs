use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowedRequest {
    Get,
    Put,
    Delete,
    List,
}

impl fmt::Display for AllowedRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllowedRequest::Get => write!(f, "GET"),
            AllowedRequest::Put => write!(f, "PUT"),
            AllowedRequest::Delete => write!(f, "DELETE"),
            AllowedRequest::List => write!(f, "LIST"),
        }
    }
}

impl AllowedRequest {
    pub fn from_str_slice(request: &str) -> Option<Self> {
        match request {
            req if req.starts_with("PUT") => Some(AllowedRequest::Put),
            req if req.starts_with("LIST") => Some(AllowedRequest::List),
            req if req.starts_with("DELETE") => Some(AllowedRequest::Delete),
            req if req.starts_with("GET") => Some(AllowedRequest::Get),
            _ => None,
        }
    }
}
