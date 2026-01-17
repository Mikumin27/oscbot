use poise::ChoiceParameter;

use crate::{Error};

#[derive(Debug, Clone, Copy, PartialEq, ChoiceParameter, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum DEFAULT {
    #[name = "Default"]
    DEFAULT,
    #[name = "NM"]
    NM,
    #[name = "HD"]
    HD,
    #[name = "DT"]
    DT,
    #[name = "HR"]
    HR,
    #[name = "EZ"]
    EZ,
    #[name = "HDDT"]
    HDDT,
    #[name = "HDHR"]
    HDHR,
    #[name = "No default"]
    NODEFAULT,
}

impl DEFAULT {
    pub fn to_string(self) -> String {
        match self {
            DEFAULT::DEFAULT => "Default".to_string(),
            DEFAULT::NM => "NM".to_string(),
            DEFAULT::HD => "HD".to_string(),
            DEFAULT::DT => "DT".to_string(),
            DEFAULT::HR => "HR".to_string(),
            DEFAULT::EZ => "EZ".to_string(),
            DEFAULT::HDDT => "HDDT".to_string(),
            DEFAULT::HDHR => "HDHR".to_string(),
            DEFAULT::NODEFAULT => "No default".to_string(),
        }
    }

    pub fn to_db(self) -> Option<String> {
        if self == DEFAULT::NODEFAULT {
            return None;
        }

        Some(self.to_string())
    }

    pub fn from_string(value: String) -> DEFAULT {
        match value.as_str() {
            "Default" => DEFAULT::DEFAULT,
            "NM" => DEFAULT::NM,
            "HD" => DEFAULT::HD,
            "DT" => DEFAULT::DT,
            "HR" => DEFAULT::HR,
            "EZ" => DEFAULT::EZ,
            "HDDT" => DEFAULT::HDDT,
            "HDHR" => DEFAULT::HDHR,
            _ => DEFAULT::NODEFAULT,
        }
    }

    pub fn from_db(value: Option<String>) -> DEFAULT {
        match value {
            Some(value) => DEFAULT::from_string(value),
            None => DEFAULT::NODEFAULT
        }
    }
}

pub async fn download(url: &String) -> Result<Option<Vec<u8>>, Error> {
    let client = reqwest::Client::new();
    let resp = match client.get(url).send().await?.error_for_status() {
        Ok(response) => response,
        Err(_) => return Ok(None)
    };

    let bytes = match resp.bytes().await {
        Ok(bytes) => bytes.to_vec(),
        Err(_) =>  return Ok(None)
    };

    Ok(Some(bytes))
}