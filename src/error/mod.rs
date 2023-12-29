pub type ClioError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type ClioResult<T> = Result<T, ClioError>;
