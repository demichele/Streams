use anyhow::Result;

//import dependency for base64 encoding
use base64::{encode_config, URL_SAFE_NO_PAD};

// import the IOTA-Streams Dependencies
use iota_streams::ddml::types::Bytes;
use iota_streams::{
    app::transport::tangle::client::Client as IotaClient,
    app_channels::api::tangle::{Address, Author},
};

// Writes a new message into the channel, linking it to the previous one
pub fn send_public_msg(
    data: &str,
    previous_msg_link: Address,
    author: &mut Author<IotaClient>,
) -> Result<String> {
    //encode the data string into base64
    let encoded_data = encode_config(&data, URL_SAFE_NO_PAD);

    //transform the base64 data to bytes
    let data_bytes = Bytes(encoded_data.as_bytes().to_vec());

    // We don't need a masked payload, so we give set it to empty bytes
    let masked_payload = &Bytes::default();

    //Send the data bytes to the channel as a public payload, linked to the previous message
    let signed_packet_message =
        author.send_signed_packet(&previous_msg_link, &data_bytes, masked_payload)?;

    // Get and return the id of the new message to generate the next link
    let message_id = signed_packet_message.0.msgid.to_string();
    Ok(message_id)
}
