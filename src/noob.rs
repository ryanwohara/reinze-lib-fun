use rand::{self, Rng};

pub fn noob(query: String, author: String) -> Result<Vec<String>, String> {
    let mut split = query.split(" ");
    let first_token = split.next().unwrap_or_default();

    let mut nick = author.split("!").collect::<Vec<&str>>()[0];

    if first_token.len() > 0 {
        nick = first_token;
    }

    let output = match rand::rng().random_range(0..=1) {
        0 => "is not",
        _ => "is",
    };

    Ok(vec![format!("{nick} {output} a silly noob!")])
}
