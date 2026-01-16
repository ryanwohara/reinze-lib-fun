use common::{c2, l};
use serde::{Deserialize, Serialize};

pub fn lookup() -> Result<Vec<String>, String> {
    let response = match reqwest::blocking::get("https://official-joke-api.appspot.com/random_joke")
    {
        Ok(resp) => match resp.json::<JokeResponse>() {
            Ok(string) => string,
            Err(e) => {
                println!("Error getting text: {}", e);
                return Ok(vec!["Joke API seems broken!".to_string()]);
            }
        },
        Err(e) => {
            println!("Error making HTTP request: {}", e);
            return Ok(vec!["Joke API is down!".to_string()]);
        }
    };

    let setup = vec![l("Joke"), c2(&response.setup)].join(" ");
    let punchline = vec![l("Answer"), c2(&response.punchline)].join(" ");

    Ok(vec![setup, punchline])
}

#[derive(Debug, Serialize, Deserialize)]
struct JokeResponse {
    #[serde(alias = "type")]
    _type: String,
    setup: String,
    punchline: String,
    id: i32,
}
