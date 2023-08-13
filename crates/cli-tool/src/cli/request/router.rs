use crate::cli::args::RequestArgs;

use super::{get::get, post::post};

/// request 的方法
pub enum RequestMethod {
    Post,
    Get,
}

impl From<String> for RequestMethod {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Post" => Self::Post,
            "post" => Self::Post,
            _ => Self::Get,
        }
    }
}

impl RequestMethod {
    fn new(value: &String) -> Self {
        match value.as_str() {
            "Post" => Self::Post,
            "post" => Self::Post,
            _ => Self::Get,
        }
    }
}

pub async fn create_request(request_args: &RequestArgs) {
    let method = RequestMethod::new(&request_args.method);
    match method {
        RequestMethod::Get => match get(&request_args.url).await {
            Err(e) => {
                println!("{:?}", e);
            }
            Ok(_) => {}
        },
        RequestMethod::Post => match post(&request_args.data, &request_args.url).await {
            Err(e) => {
                println!("{:?}", e);
            }
            Ok(_) => {}
        },
    }
}
