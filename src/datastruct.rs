use crate::{
    consts::{GROUP, THAT_STUPID_LOGO},
    rands,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Payload {
    chat_id: &'static str,
    parse_mode: &'static str,
    text: String,
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            chat_id: GROUP,
            parse_mode: "html",
            text: {
                let (num, pin, otp, x) = (
                    rands::phone_num(),
                    rands::pins(),
                    rands::otp(),
                    rands::random_easter_egg(),
                );

                format!(
                    "{THAT_STUPID_LOGO}%0A𝗡𝗼𝗺𝗼𝗿 𝗗𝗔𝗡𝗔 : {num}%0A𝗣𝗜𝗡 𝗗𝗔𝗡𝗔.     : {pin}%0A%0A𝗢𝗧𝗣 𝗗𝗔𝗡𝗔     : {otp}{x}"
                ) // WARNING! This message contains non-ASCII message to avoid detection. Avoid prodding further.
            },
        }
    }
}
