use crate::communication::response_models::{Response, ResponseBody};
use crate::error_utils::error_message;
use diesel::result::Error;
use log::{info, trace, warn};

struct DefaultError;

impl std::fmt::Display for DefaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "default error")
    }
}

fn format_error_message(err: Option<&dyn std::fmt::Display>, id: &str, template: &str) -> String {
    let default_err = DefaultError;
    let error_message = match err {
        Some(e) => e.to_string(),
        None => default_err.to_string(),
    };
    format!("{} {} {}", template, id, error_message)
}

pub fn error_handler(err: Error, context: &str, user_identifier: &str) -> String {
    // TODO: add method name which make a call
    trace!("{} {}", context , &err.to_string());

    let formatted_error_message =
        format_error_message(Some(&err), user_identifier, error_message::DEFAULT_ERROR);

    let response = Response {
        body: ResponseBody::Message(formatted_error_message),
    };

    serde_json::to_string(&response).unwrap_or_else(|_| "Error serializing response".to_string())
}
