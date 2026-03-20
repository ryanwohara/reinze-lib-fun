pub fn blind() -> anyhow::Result<Vec<String>> {
    let message = vec![
        "\x030,8FLASHBANG\x038,0FLASHBANG\x030,8FLASHBANG\x038,0FLASHBANG\x030,8FLASHBANG\x038,0FLASHBANG\x030,8FLASHBANG\x038,0FLASHBANG\x030,8FLASHBANG\x038,0FLASHBANG\x030,8FLASHBANG".to_string(),
    ];

    Ok(message)
}
