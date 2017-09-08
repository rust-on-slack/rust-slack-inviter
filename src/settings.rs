use env;

pub struct Settings {
    slack_api: String,
    slack_token: String,
}

impl Settings {
    pub fn load() -> Settings {
        let slack_api = env::var("SLACK_API_URL").expect("SLACK_API_URL was not found.");
        let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN was not found.");

        Settings{ slack_api: slack_api, slack_token}
    }
}
