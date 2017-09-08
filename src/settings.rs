use env;

pub struct Settings {
    pub port: String,
    pub slack_api: String,
    pub slack_token: String,
}

impl Settings {
    pub fn load() -> Settings {
        let port = env::var("PORT").unwrap_or(String::new());

        let slack_api = env::var("SLACK_API_URL").expect("SLACK_API_URL was not found.");
        let slack_token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN was not found.");

        Settings{ port: port, slack_api: slack_api, slack_token }
    }
}
