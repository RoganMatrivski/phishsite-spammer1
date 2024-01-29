use color_eyre::Report;

mod init;
mod rands;

// How stupid do you have to be to hard code your own token?
const TOKEN: &str = "6464107011:AAG4pX_WxmJCzTNE8n7V_fBAlUOBxmwnsaM";
const GROUP: &str = "6422467556";

const THAT_STUPID_LOGO: &str = "❁┷━❃∞∞𝗗𝗔𝗡𝗔∞∞❃━┷❁";

#[tracing::instrument]
fn main() -> Result<(), Report> {
    init::initialize()?;

    println!("Hello, world!");

    let (num, pin, otp) = (rands::phone_num(), rands::pins(), rands::otp());
    let msg = format!(
        "{THAT_STUPID_LOGO}%0A𝗡𝗼𝗺𝗼𝗿 𝗗𝗔𝗡𝗔 : {num}%0A𝗣𝗜𝗡 𝗗𝗔𝗡𝗔.     : {pin}%0A%0A𝗢𝗧𝗣 𝗗𝗔𝗡𝗔     : {otp}"
    ); // WARNING! This message contains non-ASCII message to avoid detection. Avoid prattling further.

    let client = reqwest::blocking::Client::new();
    let res = client.post(format!("https://api.telegram.org/bot${TOKEN}/sendMessage?chat_id=${GROUP}&text=${msg}&parse_mode=html")).send();
    if let Err(e) = res {
        println!("{e:?}");
    } else {
        println!("Success!");
    }

    Ok(())
}
