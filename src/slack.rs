use reqwest;
use std::io::Read;

pub struct SlackClient {
    url: String,
    token: String,
}

impl SlackClient {
    pub fn new(api: String, token: String) -> SlackClient {
        SlackClient {
            url: api,
            token: token,
        }
    }

    pub fn invite(&self, email: &str) -> String {
        let params = [
            ("email", email.as_ref()),
            ("token", self.token.as_ref()),
            ("set_active", "true"),
        ];

        let client = reqwest::Client::new().unwrap();
        let mut res = client.post(self.endpoint("/api/users.admin.invite")).unwrap()
                        .form(&params).unwrap()
                        .send().unwrap();

        let mut content = String::new();
        match res.read_to_string(&mut content) {
            Err(err) => println!("Error {}", err),
            _ => ()
        }

        content
    }

    fn endpoint(&self, uri: &str) -> reqwest::Url {
        let url = String::from(self.url.clone() + uri);

        reqwest::Url::parse(&url[..])
            .expect("Api url is invalid")
    }
}
