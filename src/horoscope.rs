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
        s.c2(&response.data.horoscope),
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

/// Only the parts of the response this command actually reads.
///
/// It previously also required `status`, `success` and `date`, none of which
/// were ever used. When the API dropped the first two and renamed
/// `horoscope_data` to `horoscope`, deserialisation failed, `lookup` returned
/// an error, and the host turned that into no output at all -- a command that
/// looked like it had stopped existing. Serde ignores unknown fields, so
/// naming only what is used keeps a sibling field appearing or disappearing
/// from breaking this again.
#[derive(Debug, Serialize, Deserialize)]
struct HoroscopeResponse {
    data: HoroscopeObject,
}

#[derive(Debug, Serialize, Deserialize)]
struct HoroscopeObject {
    horoscope: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shape the API actually returns, captured 2026-08-21.
    #[test]
    fn the_live_response_shape_deserialises() {
        let body = r#"{"data":{"date":"2026-08-21","period":"daily",
                       "sign":"Aries","horoscope":"Today you may feel bold."}}"#;

        let parsed: HoroscopeResponse = serde_json::from_str(body).expect("response parses");
        assert_eq!(parsed.data.horoscope, "Today you may feel bold.");
    }

    /// The regression itself: the old response carried siblings the new one
    /// does not, and a struct that demands them cannot read either shape.
    #[test]
    fn extra_and_missing_siblings_are_both_tolerated() {
        for body in [
            r#"{"data":{"horoscope":"x"}}"#,
            r#"{"data":{"horoscope":"x","date":"2026-08-21"},"status":200,"success":true}"#,
            r#"{"data":{"horoscope":"x","something_new":42},"unexpected":"field"}"#,
        ] {
            let parsed: HoroscopeResponse =
                serde_json::from_str(body).unwrap_or_else(|e| panic!("{body} failed: {e}"));
            assert_eq!(parsed.data.horoscope, "x");
        }
    }

    #[test]
    fn a_response_without_the_horoscope_text_is_an_error() {
        let body = r#"{"data":{"date":"2026-08-21"}}"#;
        assert!(serde_json::from_str::<HoroscopeResponse>(body).is_err());
    }

    #[test]
    fn parse_accepts_full_names_and_shorthands() {
        for (query, expected) in [
            ("aries", "Aries"),
            ("ARIES", "Aries"),
            ("aqua", "Aquarius"),
            ("scorp", "Scorpio"),
            ("sag", "Sagittarius"),
            ("cap", "Capricorn"),
            ("pisc", "Pisces"),
        ] {
            let sign = Horoscope::parse(query).unwrap_or_else(|_| panic!("{query} should parse"));
            assert_eq!(sign.to_string(), expected);
        }
    }

    #[test]
    fn parse_rejects_an_unknown_sign() {
        assert!(Horoscope::parse("ophiuchus").is_err());
        assert!(Horoscope::parse("").is_err());
    }
}
