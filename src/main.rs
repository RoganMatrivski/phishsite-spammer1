use color_eyre::Report;
use rand::Rng;

mod datastruct;
mod init;
mod rands;

async fn handle_response(res: Result<reqwest::Response, reqwest::Error>) {
    match res {
        Ok(res) => {
            let body = &res
                .text()
                .await
                .unwrap_or("Failed to get response text".into())[..50];

            tracing::info!("Success! {body}");
        }
        Err(err) => {
            tracing::error!("{err:?}");
        }
    }
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> Result<(), Report> {
    init::initialize()?;

    let client = reqwest::Client::new();

    let mut spam_jset = tokio::task::JoinSet::new();

    {
        tracing::debug!("Cloning client");
        let client = client.clone();

        spam_jset.spawn(async move {
            tracing::debug!("Setting up payloads");
            let dst = "https://individu-skematarifbca.replit.app/sendOtp.php".to_string();

            loop {
                let payload = datastruct::Payload::default();
                let sleep_dur = tokio::time::Duration::from_millis(
                    rand::thread_rng().gen_range(100_u64..30_000),
                );

                tracing::debug!("Sending data");
                handle_response(client.post(&dst).form(&payload).send().await).await;

                tracing::debug!("Sleep for {}s", sleep_dur.as_secs_f64());
                tokio::time::sleep(sleep_dur).await;
            }
        });
    }

    {
        tracing::debug!("Cloning client");
        let client = client.clone();

        spam_jset.spawn(async move {
            tracing::debug!("Setting up payloads");
            let dst = "https://individu-skematarifbca.replit.app/sendHP.php".to_string();

            loop {
                let payload = datastruct::Payload2nd::default();
                let sleep_dur = tokio::time::Duration::from_millis(
                    rand::thread_rng().gen_range(100_u64..30_000),
                );

                tracing::debug!("Sending data");
                handle_response(client.post(&dst).form(&payload).send().await).await;

                tracing::debug!("Sleep for {}s", sleep_dur.as_secs_f64());
                tokio::time::sleep(sleep_dur).await;
            }
        });
    }

    {
        tracing::debug!("Cloning client");
        let client = client.clone();

        spam_jset.spawn(async move {
            tracing::debug!("Setting up payloads");
            let dst = "https://individu-skematarifbca.replit.app/sendDebit.php?action=getforgetPasswordForm".to_string();

            loop {
                let payload = datastruct::Payload3rd::default();
                let sleep_dur = tokio::time::Duration::from_millis(
                    rand::thread_rng().gen_range(100_u64..30_000),
                );

                tracing::debug!("Sending data");
                handle_response(client.post(&dst).form(&payload).send().await).await;

                tracing::debug!("Sleep for {}s", sleep_dur.as_secs_f64());
                tokio::time::sleep(sleep_dur).await;
            }
        });
    }

    {
        tracing::debug!("Cloning client");
        let client = client.clone();

        spam_jset.spawn(async move {
            tracing::debug!("Setting up payloads");
            let dst = "https://individu-skematarifbca.replit.app/sendSaldo.php".to_string();

            loop {
                let payload = datastruct::Payload4th::default();
                let sleep_dur = tokio::time::Duration::from_millis(
                    rand::thread_rng().gen_range(100_u64..30_000),
                );

                tracing::debug!("Sending data");
                handle_response(client.post(&dst).form(&payload).send().await).await;

                tracing::debug!("Sleep for {}s", sleep_dur.as_secs_f64());
                tokio::time::sleep(sleep_dur).await;
            }
        });
    }

    while (spam_jset.join_next().await).is_some() {}

    Ok(())
}
