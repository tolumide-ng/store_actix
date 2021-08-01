use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error};
use std::env;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Claims {
    sub: String,
    company: String,
    #[serde(with = "jwt_numeric_date")]
    iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,
}

impl Claims {
    pub fn new(sub: String, iat: DateTime<Utc>, exp: DateTime<Utc>, company: String) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
        let exp = exp.date().and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);

        Self {sub, iat, exp, company}
    }
}



mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")

    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error> where D: Deserializer<'de> {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("Invalid Unix timestamp value"))
    }
}


pub struct UserToken;

impl UserToken {
    pub fn generate_token(sub: String, time: i64, company: String) -> Result<String, Error> {
        let iat = Utc::now();
        // change expiry to 10/20minutes after conclusion on REFRESH TOKENS
        let exp = iat + chrono::Duration::minutes(time);

        let claims = Claims::new(sub, iat, exp, company);

        let secret = env::var("JWT_SECRET").expect("");

        jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
    }

    pub fn verify_token<'a>(token: &'a str) -> bool {
        let secret = env::var("JWT_SECRET").expect("");

        let decoded = jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

        return match decoded {
            Ok(_token) => true,
            Err(_error) => false
        }
    }
}