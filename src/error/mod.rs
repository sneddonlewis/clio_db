pub type ClioError = Box<dyn std::error::Error>;
pub type ClioResult<T> = Result<T, ClioError>;
