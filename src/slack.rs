use reqwest;

pub struct SlackClient {
    api_url: String,
    token: String,
}

impl SlackClient {
    pub fn new(api: String, token: String) -> SlackClient {
        SlackClient {
            api_url: api,
            api_token: token,
        }
    }

    pub fn invite(&self, email: String) -> String {
        let url = reqwest::Url::parse(&self.api_url[..]).expect("Api url is invalid");
        let params = [
            ("email", email.as_ref()),
            ("token", api_token.as_ref()),
            ("set_active", "true"),
        ];

        let client = reqwest::Client::new().unwrap();
        let mut res = client.post(url).unwrap()
                        .form(&params).unwrap()
                        .send().unwrap();

        let mut content = String::new();
        res.read_to_string(&mut content);

        content
    }
}
