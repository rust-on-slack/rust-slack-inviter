extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate reqwest;
extern crate dotenv;
extern crate serde_json;

mod settings;

use std::io::Read;

use dotenv::dotenv;
use std::env;
use std::path::Path;

use std::str::FromStr;
use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use iron::{Iron, Request, Response, IronResult, IronError};
use iron::status;

use serde_json::{Value, Error};

fn mount_slack_api_url() -> reqwest::Url {
    let slack_url = env::var("SLACK_API_URL")
        .expect("SLACK_API_URL was not found.");

    let api_url = String::from(slack_url + "/api/users.admin.invite");

    let url = reqwest::Url::parse(&api_url[..]).expect("Api url is invalid");
    return url
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn server_port() -> u16 {
    let port_str = env::var("PORT").unwrap_or(String::new());
    FromStr::from_str(&port_str).unwrap_or(3000)
}

fn handle_invite(req: &mut Request) -> IronResult<Response> {
    let api_token = env::var("SLACK_API_TOKEN")
        .expect("SLACK_API_TOKEN was not found.");

    let mut payload = String::new();
    req.body.read_to_string(&mut payload).unwrap();

    let v: Value = serde_json::from_str(&payload[..]).unwrap();
    println!("params {}", v);

    let params = [
        ("token", api_token.to_owned()),
        ("email", v["email"].to_string()),
        ("set_active", "true".to_owned())
    ];

    let client = reqwest::Client::new().unwrap();
    let mut res = client.post(mount_slack_api_url()).unwrap()
                    .form(&params).unwrap()
                    .send().unwrap();
    let mut content = String::new();
    res.read_to_string(&mut content);

    Ok(Response::with((status::Ok, content)))
}

fn main() {
    dotenv().ok();

    let mut mount = Mount::new();
    let port = server_port();

    mount.mount("/", Static::new(Path::new("static/")));
    mount.mount("/invite", handle_invite);


    println!("Running on: http://localhost:{}", port);
    Iron::new(mount).http(("0.0.0.0", port)).unwrap();
}
