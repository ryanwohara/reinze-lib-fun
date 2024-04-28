use rand::{self, Rng};

pub fn noob(query: String, mut author: String) -> Result<Vec<String>, String> {
    let mut split = query.split(" ");
    let first_token = split.next().unwrap_or_default();

    if first_token.len() > 0 {
        author = first_token.to_owned();
    }

    let mut r = rand::thread_rng();
    let output = match r.gen_range(0..=1) {
        i32::MIN..=-1_i32 => "is",
        2_i32..=i32::MAX => "is not",
        _ => "is not",
    };

    Ok(vec![format!("{author} {output} a silly noob!")])
}
