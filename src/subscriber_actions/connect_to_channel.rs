use anyhow::Result;

// import the IOTA-Streams Dependencies
use iota_streams::app::transport::tangle::client::Client as IotaClient;
use iota_streams::app_channels::api::tangle::{Address, Subscriber};

//Connects to a channel by findign the annouce message from it's link and digesting the author public key
pub fn connect(subscriber: &mut Subscriber<IotaClient>, announcement_link: Address) -> Result<()> {
    subscriber.receive_announcement(&announcement_link)?;
    Ok(())
}
