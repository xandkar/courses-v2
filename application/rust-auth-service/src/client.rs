use std::net::SocketAddr;

use crate::protocol;

#[tracing::instrument(skip_all)]
pub async fn echo(addr: SocketAddr, text: &str) -> anyhow::Result<()> {
    let url = format!("http://{addr}/echo");
    let req_msg = protocol::Echo {
        text: text.to_string(),
    };
    tracing::debug!(?url, ?req_msg, "Sending request.");
    let resp_msg = reqwest::Client::new()
        .post(url)
        .json(&req_msg)
        .send()
        .await?
        .json::<protocol::Echo>()
        .await?;
    tracing::debug!(?resp_msg, "Received response.");
    assert_eq!(req_msg, resp_msg);
    println!("{}", resp_msg.text);
    Ok(())
}
