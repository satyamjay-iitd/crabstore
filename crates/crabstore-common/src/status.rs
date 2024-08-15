use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusCode {
    OK,
    OutOfMemory,
    KeyError,
    ObjectRefEndOfStream,
    TypeError,
    OutOfDisk,
    Invalid,
    IOError,
    InvalidArgument,
}

#[derive(Debug, Clone)]
pub struct Status {
    state: Option<Box<State>>,
}

#[derive(Debug, Clone)]
struct State {
    code: StatusCode,
    msg: String,
    rpc_code: i32,
}

impl Status {
    pub fn from_error(code: StatusCode, msg: String, rpc_code: i32) -> Self {
        Status {
            state: Some(Box::new(State {
                code,
                msg,
                rpc_code,
            })),
        }
    }

    // Return a success status.
    pub fn ok() -> Self {
        Status { state: None }
    }

    // Return error status of an appropriate type.
    pub fn out_of_memory(msg: String) -> Self {
        Status::from_error(StatusCode::OutOfMemory, msg, -1)
    }

    pub fn key_error(msg: String) -> Self {
        Status::from_error(StatusCode::KeyError, msg, -1)
    }

    pub fn object_ref_end_of_stream(msg: String) -> Self {
        Status::from_error(StatusCode::ObjectRefEndOfStream, msg, -1)
    }

    pub fn type_error(msg: String) -> Self {
        Status::from_error(StatusCode::TypeError, msg, -1)
    }

    pub fn is_ok(&self) -> bool {
        self.state.is_none()
    }

    pub fn is_out_of_memory(&self) -> bool {
        self.code() == StatusCode::OutOfMemory
    }

    pub fn is_key_error(&self) -> bool {
        self.code() == StatusCode::KeyError
    }

    pub fn is_object_ref_end_of_stream(&self) -> bool {
        self.code() == StatusCode::ObjectRefEndOfStream
    }

    pub fn is_type_error(&self) -> bool {
        self.code() == StatusCode::TypeError
    }

    pub fn code(&self) -> StatusCode {
        self.state
            .as_ref()
            .map_or(StatusCode::OK, |s| s.code.clone())
    }

    pub fn rpc_code(&self) -> i32 {
        self.state.as_ref().map_or(-1, |s| s.rpc_code)
    }

    pub fn message(&self) -> String {
        self.state.as_ref().map_or(String::new(), |s| s.msg.clone())
    }

    pub fn code_as_string(&self) -> String {
        match self.code() {
            StatusCode::OK => "OK".to_string(),
            StatusCode::OutOfMemory => "OutOfMemory".to_string(),
            StatusCode::KeyError => "KeyError".to_string(),
            StatusCode::ObjectRefEndOfStream => "ObjectRefEndOfStream".to_string(),
            StatusCode::TypeError => "TypeError".to_string(),
            StatusCode::OutOfDisk => "OutOfDisk".to_string(),
            StatusCode::Invalid => "Invalid".to_string(),
            StatusCode::IOError => "IOError".to_string(),
            StatusCode::InvalidArgument => "InvalidArgument".to_string(),
            // Add more cases here as needed
        }
    }
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_ok() {
            write!(f, "OK")
        } else {
            write!(f, "{}: {}", self.code_as_string(), self.message())
        }
    }
}
