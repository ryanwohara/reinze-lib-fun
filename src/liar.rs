extern crate common;

pub fn liar(query: String, mut author: String) -> Result<Vec<String>, String> {
    let mut split = query.split(" ");
    let first_token = split.next().unwrap_or_default();

    if first_token.len() > 0 {
       author = first_token.to_owned();
    }

    let message = vec!["{author} is a bloody LIAR!".to_string()];
    Ok(message)
}
