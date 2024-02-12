use crate::rands;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    tele: String,
    namanye: String,
    kertu: String,
    uang: String,
    otp: String,
    captcha: String,
}

impl Default for Payload {
    fn default() -> Self {
        use fake::faker::name::raw::*;
        use fake::locales::*;
        use fake::Fake;
        use rusty_money::{iso, Money};

        Self {
            tele: rands::phone_num(),
            namanye: Name(EN).fake(),
            kertu: rands::card_num(),
            uang: Money::from_major(
                rand::thread_rng().gen_range(50_000_i64..20_000_000),
                iso::IDR,
            )
            .to_string(),
            otp: rands::otp(),
            captcha: "".into(),
        }
    }
}
