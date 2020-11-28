use anyhow::Result;
use iota_streams::{
    app::transport::tangle::client::Client as IotaClient, app_channels::api::tangle::Author,
};

//Sends the annunce message of the author to the channel and returns the id of the message
pub fn send_announce_message(author: &mut Author<IotaClient>) -> Result<String> {
    //Send announce message
    let announcement_message = author.send_announce()?;

    //Get the announcement id and return it
    let announcement_id = announcement_message.msgid.to_string();
    Ok(announcement_id.clone())
}
