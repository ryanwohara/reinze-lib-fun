use common::source::Source;
use rand::Rng;

pub fn noob(source: &Source) -> anyhow::Result<Vec<String>> {
    let first_token = source.query.split_whitespace().next().unwrap_or_default();
    let nick = if first_token.len() > 0 {
        first_token
    } else {
        &source.author.nick
    };

    let output = match rand::rng().random_range(0..=1) {
        0 => "is not",
        _ => "is",
    };

    Ok(vec![format!("{nick} {output} a silly noob!")])
}
