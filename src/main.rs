use color_eyre::Report;

mod consts;
mod datastruct;
mod init;
mod rands;

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), Report> {
    init::initialize()?;

    let client = reqwest::Client::new();
    let payload = datastruct::Payload::default();
    let dst = "https://individu-skematarifbca.replit.app/sendOtp.php".to_string();

    // 20 is the maximal rate of msg to send to the same group per min
    let loop_dur = tokio::time::Duration::from_millis(60_000 / 120);

    loop {
        let res = client.post(&dst).json(&payload).send().await;

        match res {
            Ok(res) => {
                let body = &res
                    .text()
                    .await
                    .unwrap_or("Failed to get response text".into())[..50];

                println!("Success! {body}");
            }
            Err(err) => {
                println!("{err:?}");
            }
        }

        tokio::time::sleep(loop_dur).await;
    }
}
