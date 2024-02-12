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

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload2nd {
    radio: &'static str,
    nohp: String,
    captcha: String,
}

impl Default for Payload2nd {
    fn default() -> Self {
        Self {
            radio: "on",
            nohp: rands::phone_num(),
            captcha: "".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload3rd {
    nomorku: String,
    nama: String,
    debit: String,
    captcha: String,
}

impl Default for Payload3rd {
    fn default() -> Self {
        use fake::faker::name::raw::*;
        use fake::locales::*;
        use fake::Fake;

        Self {
            nomorku: rands::phone_num(),
            nama: Name(EN).fake(),
            debit: rands::card_num(),
            captcha: "".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload4th {
    nomorsaya: String,
    namasaya: String,
    debitku: String,
    saldo: String,
    captcha: String,
}

impl Default for Payload4th {
    fn default() -> Self {
        use fake::faker::name::raw::*;
        use fake::locales::*;
        use fake::Fake;
        use rusty_money::{iso, Money};

        Self {
            nomorsaya: rands::phone_num(),
            namasaya: Name(EN).fake(),
            debitku: rands::card_num(),
            saldo: Money::from_major(
                rand::thread_rng().gen_range(50_000_i64..20_000_000),
                iso::IDR,
            )
            .to_string(),
            captcha: "".into(),
        }
    }
}
