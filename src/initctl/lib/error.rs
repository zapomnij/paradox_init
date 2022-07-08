pub enum Error {
    OperationFailed(String),

    InitctlNotFound(String),

    ServiceNotFound((String, String)),
    InitListNotFound(String),

    Other(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InitctlNotFound(e) => return write!(f, "Init doesn't expose API. /run/init/initctl isn't accessible: {e}"),
            Error::InitListNotFound(e) => return write!(f, "List of enabled services isn't accessible: {e}"),
            Error::ServiceNotFound((name, error)) => return write!(f, "Service {name} isn't accessible: {error}"),
            Error::Other(data) => return write!(f, "An error has occured: {data}"),
            Error::OperationFailed(data) => return write!(f, "Failed to do an operation: {data}"),
        }
    }
}