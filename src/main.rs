extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate reqwest;
extern crate dotenv;
extern crate serde_json;

mod settings;
mod slack;

use std::io::Read;

use dotenv::dotenv;
use std::env;
use std::path::Path;

use mount::Mount;
use staticfile::Static;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use std::str::FromStr;

use serde_json::{Value};

use slack::SlackClient;
use settings::Settings;

fn handle_invite(client: &SlackClient, req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    match req.body.read_to_string(&mut payload) {
        Ok(_) => (),
        Err(err) => println!("Error {}", err)
    }

    let v: Value = match serde_json::from_str(&payload[..]) {
        Ok(val) => val,
        Err(err) => {
            println!("Error {}", err);
            Value::Null
        }
    };

    let response = match v {
        Value::Object(params) =>
            match params.get("email") {
                Some(email) =>
                    client.invite(email.as_str().unwrap()),
                None =>
                    String::from("{ \"ok\": false, \"error\": \"invalid_email\" }"),
            },
        _ =>
            String::from("{ \"ok\": false, \"error\": \"application_error\" }"),
    };

    Ok(Response::with((status::Ok, response)))
}

fn main() {
    dotenv().ok();

    let mut mount = Mount::new();
    let settings = Settings::load();
    let slack = SlackClient::new(settings.slack_api, settings.slack_token);

    mount.mount("/", Static::new(Path::new("static/")));
    mount.mount("/invite", move |r: &mut Request| { handle_invite(&slack, r) });

    let port = FromStr::from_str(&settings.port).unwrap_or(3000);

    println!("Running on: http://localhost:{}", port);
    Iron::new(mount).http(("0.0.0.0", port)).unwrap();
}
