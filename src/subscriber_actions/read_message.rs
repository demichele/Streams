use anyhow::Result;

// import dependency for base64 decoding
use base64::{decode_config, URL_SAFE_NO_PAD};

// import the IOTA-Streams Dependencies
use iota_streams::app::transport::tangle::client::Client as IotaClient;
use iota_streams::app_channels::api::tangle::{Address, Subscriber};

// Reads a message out of the channel, needs the link to the message to find it
pub fn read_message(
    subscriber: &mut Subscriber<IotaClient>,
    signed_packet_link: Address,
) -> Result<String> {
    //Recieve the message and extract only the public payload
    let (_, public_payload, _) = subscriber.receive_signed_packet(&signed_packet_link).unwrap();

    //Transform the bytes of the message back into a string by decoding the base64
    let message = unwrap_data(&String::from_utf8(public_payload.0).unwrap()).unwrap();

    //return the message as a String
    Ok(message)
}

// unwraps the IOTA Bytes and decodes the base64 message back into a String
pub fn unwrap_data(data: &str) -> Result<String> {
    let raw = &data.to_string();
    let decode_data = decode_config(&raw, URL_SAFE_NO_PAD).unwrap();
    Ok(String::from_utf8(decode_data).unwrap())
}
