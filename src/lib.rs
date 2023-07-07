use std::env;

use rand::seq::SliceRandom;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::{json, Value as JSONValue};

#[derive(Clone, Copy)]
enum ProjectName {
    Icelandic,
    Norse,
    Swedish,
    Norwegian,
    Danish,
}

impl ProjectName {
    fn random() -> Self {
        let variants = [
            ProjectName::Icelandic,
            ProjectName::Norse,
            ProjectName::Swedish,
            ProjectName::Norwegian,
            ProjectName::Danish,
        ];
        let mut rng = rand::thread_rng();
        *variants.choose(&mut rng).unwrap()
    }

    fn as_str(&self) -> &str {
        match self {
            ProjectName::Icelandic => "old-icelandic",
            ProjectName::Norse => "old-norse",
            ProjectName::Swedish => "old-swedish",
            ProjectName::Norwegian => "old-norwegian",
            ProjectName::Danish => "old-danish",
        }
    }
}

fn get_random_path() -> String {
    let mut rng = rand::thread_rng();
    let letter: u8 = rng.gen_range(b'a'..=b'z');
    let slug = String::from_utf8(vec![letter]).unwrap();

    format!("/letter/{}/", slug)
}

fn get_random_user_id() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=1000).to_string()
}

pub fn get_request_payload() -> JSONValue {
    let project_name = ProjectName::random();
    let user_identifier = get_random_user_id();
    let path = get_random_path();

    json!({
        "projectName": project_name.as_str(),
        "path": path,
        "userIdentifier": user_identifier
    })
}

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}

pub fn get_number_of_requests() -> u32 {
    const DEFAULT_AMOUNT: u32 = 100;
    let args: Vec<String> = env::args().collect();

    let requests: u32 = match args.get(1) {
        Some(arg) => arg.parse().unwrap_or(DEFAULT_AMOUNT),
        None => DEFAULT_AMOUNT,
    };

    requests
}
