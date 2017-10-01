
#[derive(Serialize)]
pub struct Response {
    ok: bool,
    error: Option<ErrorType>,
}

#[derive(Serialize)]
pub enum ErrorType {
    InvalidEmail,
    ApplicationError,
}

impl Response {
    pub fn new(ok: bool, error: Option<ErrorType>) -> Response {
        Response {
            ok,
            error
        }
    }
}
