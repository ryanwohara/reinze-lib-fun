use common::{c2, l, p};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub fn lookup(query: String) -> Result<Vec<String>, String> {
    let sign = match Horoscope::parse(&query) {
        Ok(sign) => sign,
        Err(error) => return Ok(vec![error.to_string()]),
    };

    let response = match reqwest::blocking::get(format!(
        "https://horoscope-app-api.vercel.app/api/v1/get-horoscope/daily?day=TODAY&sign={}",
        sign
    )) {
        Ok(resp) => match resp.json::<HoroscopeResponse>() {
            Ok(string) => string,
            Err(e) => {
                println!("Error getting text: {}", e);
                return Ok(vec!["Horoscope API seems broken!".to_string()]);
            }
        },
        Err(e) => {
            println!("Error making HTTP request: {}", e);
            return Ok(vec!["Horoscope API is down!".to_string()]);
        }
    };

    let output = vec![
        l("Horoscope"),
        p(sign.to_string().as_str()),
        c2(&response.data.horoscope_data),
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
    fn parse(query: &str) -> Result<Self, String> {
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
            _ => Err(format!("Unknown Horoscope query: {}", query)),
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
