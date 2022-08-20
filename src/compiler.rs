#[derive(Debug)]
pub struct CompileError {
    pub error_type: ErrorType,
    pub location: usize,
}

#[derive(Debug)]
pub enum ErrorType {
    InvalidNumError,
    InvalidTokenError,
    InternalError,
}