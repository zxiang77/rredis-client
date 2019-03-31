pub type RedisResult<T> = Result<T, RedisError>;

pub struct RedisError {
    rpr: ErrorRpr
}

#[derive(Debug)]
enum ErrorRpr {
    /// fill in later
}