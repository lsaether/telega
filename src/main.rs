extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Read;

use serde_json::Value;


fn main() {
    let bot_token = "547846290:AAFwQk6rAO7_77Yn1aEF5CLojLoIP12jhYs";
    println!("My bot token: {}", bot_token);
    println!("Booting bot..");
    let bot = Bot::boot(&bot_token);
    bot.get_me();
}

/// User type
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    is_bot: bool,
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    username: String,
    #[serde(default)]
    language_code: String,
}

struct Bot {
    token: String,
    reqwest_client: reqwest::Client,
}

impl Bot {
    fn boot(api_token: &str) -> Bot {
        Bot {
            token: api_token.to_string(),
            reqwest_client: reqwest::Client::new(),
        }
    }

    fn form_method(&self, method: &str) -> String {
        let req = format!("https://api.telegram.org/bot{}/{}", self.token, method);
        req
    }

    fn handle_response(res: reqwest::Response) -> Result<reqwest::Response, &'static str> {
        if res.status().is_success() {
            Ok(res)
        } else if res.status().is_server_error() {
            Err("Server error!")
        } else {
            Err("Something went wrong!")
        }
    }

    fn get_me(&self) {
        let req = Bot::form_method(&self, "getMe"); 
        let res = self.reqwest_client.get(&req).send().unwrap();
        match Bot::handle_response(res) {
            Ok(mut res) => {
                let mut content = String::new();
                res.read_to_string(&mut content).unwrap();
                println!("{:?}", &content);
                let value: Value = serde_json::from_str(&content).unwrap();
                let me: User = serde_json::from_str(&value["result"].to_string()).unwrap();
                println!("{:?}", me);
            },
            Err(e) => panic!("{:?}", e),
        }
    }


}
