use anyhow::Context;
use common::source::Source;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub fn lookup(s: &Source) -> anyhow::Result<Vec<String>> {
    let sign = Horoscope::parse(&s.query)?;

    let response = reqwest::blocking::get(format!(
        "https://horoscope-app-api.vercel.app/api/v1/get-horoscope/daily?day=TODAY&sign={}",
        sign
    ))
    .context("Horoscope API is down")?
    .json::<HoroscopeResponse>()
    .context("Horoscope API returned unexpected response")?;

    let output = vec![
        s.l("Horoscope"),
        s.p(sign.to_string().as_str()),
        s.c2(&response.data.horoscope_data),
    ]
    .join(" ");

    Ok(vec![output])
}

enum Horoscope {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl Horoscope {
    fn parse(query: &str) -> anyhow::Result<Self> {
        match query.to_lowercase().as_str() {
            "aries" => Ok(Self::Aries),
            "taurus" => Ok(Self::Taurus),
            "gemini" => Ok(Self::Gemini),
            "cancer" => Ok(Self::Cancer),
            "leo" => Ok(Self::Leo),
            "virgo" => Ok(Self::Virgo),
            "libra" => Ok(Self::Libra),
            "scorp" | "scorpio" => Ok(Self::Scorpio),
            "sag" | "sagittarius" => Ok(Self::Sagittarius),
            "cap" | "capricorn" => Ok(Self::Capricorn),
            "aqua" | "aquarius" => Ok(Self::Aquarius),
            "pisc" | "pisces" => Ok(Self::Pisces),
            _ => Err(anyhow::anyhow!("Unknown Horoscope query: {}", query)),
        }
    }
}

impl Display for Horoscope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = match self {
            Self::Aries => "Aries",
            Self::Taurus => "Taurus",
            Self::Gemini => "Gemini",
            Self::Cancer => "Cancer",
            Self::Leo => "Leo",
            Self::Virgo => "Virgo",
            Self::Libra => "Libra",
            Self::Scorpio => "Scorpio",
            Self::Sagittarius => "Sagittarius",
            Self::Capricorn => "Capricorn",
            Self::Aquarius => "Aquarius",
            Self::Pisces => "Pisces",
        };

        write!(f, "{}", sign)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HoroscopeResponse {
    data: HoroscopeObject,
    status: i32,
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct HoroscopeObject {
    date: String,
    horoscope_data: String,
}
