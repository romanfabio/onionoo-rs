use std::{str::FromStr, fmt::{self, Display}};

use serde::{de::{Visitor, self}, Deserialize, Deserializer};

#[derive(Debug)]
pub struct Fingerprint([u8; 40]);

#[derive(Debug)]
pub struct ParseFingerprintError;

impl FromStr for Fingerprint {
    type Err = ParseFingerprintError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 40 || !s.is_ascii() {
            return Err(ParseFingerprintError{});
        }

        let mut hash = Fingerprint([0; 40]);

        for (i, ch) in s.char_indices() {
            match ch {
                'A'..='F' | '0'..='9' => hash.0[i] = ch as u8,
                'a'..='f' => hash.0[i] = (ch as u8).to_ascii_uppercase(),
                _ => return Err(ParseFingerprintError{})
            }
        }

        Ok(hash)
    }
}

struct FingerprintVisitor;

impl<'de> Visitor<'de> for FingerprintVisitor {
    type Value = Fingerprint;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing 40 hex characters")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(fingerprint) = s.parse::<Fingerprint>() {
            Ok(fingerprint)
        } else {
            Err(de::Error::invalid_value(de::Unexpected::Str(s), &self))
        }
    }
}

impl<'de> Deserialize<'de> for Fingerprint {
    fn deserialize<D>(deserializer: D) -> Result<Fingerprint, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FingerprintVisitor)
    }
}

impl Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(std::str::from_utf8(&self.0).unwrap())
    }
}