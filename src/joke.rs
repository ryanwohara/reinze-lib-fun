use anyhow::Context;
use common::source::Source;
use serde::{Deserialize, Serialize};

pub fn lookup(s: &Source) -> anyhow::Result<Vec<String>> {
    let response = reqwest::blocking::get("https://official-joke-api.appspot.com/random_joke")
        .context("Joke API is down")?
        .json::<JokeResponse>()
        .context("Joke API returned unexpected response")?;

    let setup = vec![s.l("Joke"), s.c2(&response.setup)].join(" ");
    let punchline = vec![s.l("Answer"), s.c2(&response.punchline)].join(" ");

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
