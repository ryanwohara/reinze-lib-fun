extern crate common;

pub fn liar(query: String, author: String) -> Result<Vec<String>, String> {
    let mut split = query.split(" ");
    let first_token = split.next().unwrap_or_default();
    let mut nick = author.split("!").collect::<Vec<&str>>()[0];
    if first_token.len() > 0 {
        nick = first_token;
    }

    let message = format!("{nick} is a bloody LIAR!");
    Ok(vec![message])
}
