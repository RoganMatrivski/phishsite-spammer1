use phonenumber::{self, country, Mode};

use rand::{seq::SliceRandom, Rng};

const NUM_PREFIXES: [&str; 40] = [
    "0811", "0812", "0813", "0821", "0822", "0823", "0851", "0852", "0853", "0814", "0815", "0816",
    "0855", "0856", "0857", "0858", "0895", "0896", "0897", "0898", "0899", "0817", "0818", "0819",
    "0831", "0832", "0833", "0838", "0859", "0877", "0878", "0881", "0882", "0883", "0884", "0885",
    "0886", "0887", "0888", "0889",
];

pub fn phone_num() -> String {
    let mut rng = rand::thread_rng();
    let num_amt: usize = rng.gen_range(11..=13) - 4;

    let num = (0..num_amt)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect::<Vec<_>>()
        .join("");

    let random_prefix = NUM_PREFIXES.choose(&mut rng).unwrap();

    phonenumber::parse(Some(country::ID), format!("{random_prefix}{num}"))
        .unwrap()
        .format()
        .mode(Mode::National)
        .to_string()
}

pub fn pins() -> String {
    let mut rng = rand::thread_rng();

    (0..6)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect::<Vec<_>>()
        .join("")
}

pub fn otp() -> String {
    let mut rng = rand::thread_rng();

    (0..4)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect::<Vec<_>>()
        .join("")
}

const LOVE_LETTER: &str =
    "%0A%0A%0AThis spammer program was made with ❤ by RGMT. https://paste.gg/p/anonymous/a90c181f72f14575bd13b8e61047acb1. You've done this to yourself. Get fucked.";

pub fn random_easter_egg() -> &'static str {
    if rand::thread_rng().gen::<u8>() < 16 {
        LOVE_LETTER
    } else {
        ""
    }
}
