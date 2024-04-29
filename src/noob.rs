use rand::{self, Rng};

pub fn noob(query: String, mut author: String) -> Result<Vec<String>, String> {
    let mut split = query.split(" ");
    let first_token = split.next().unwrap_or_default();

    if first_token.len() > 0 {
        author = first_token.to_owned();
    }

    let mut r = rand::thread_rng();
    let output = match r.gen_range(0..=1) {
        0 => "is not",
        _ => "is",
    };

    Ok(vec![format!("{author} {output} a silly noob!")])
}
