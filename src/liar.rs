use common::source::Source;

pub fn liar(source: &Source) -> anyhow::Result<Vec<String>> {
    let first_token = source.query.split_whitespace().next().unwrap_or_default();
    let nick = if first_token.len() > 0 {
        first_token
    } else {
        &source.author.nick
    };

    Ok(vec![format!("{nick} is a bloody LIAR!")])
}
