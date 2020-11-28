//Import the IOTA-Streams Dependencies
use iota_streams::{
    app::transport::{
        tangle::{
            client::{Client as IotaClient, SendTrytesOptions},
            PAYLOAD_BYTES,
        },
        TransportOptions,
    },
    app_channels::api::tangle::{Address, Author},
};
//Import our code for the Author
use example::{
    author_actions::{announce::send_announce_message, send_public_msg::send_public_msg},
    random_seed,
};

//Main entry point of the programm
fn main() {
    //Generate a new random seed
    let seed = random_seed();

    //Create an IOTA client instance, this is our connection to the IOTA-Network
    let mut client = IotaClient::new_from_url("https://nodes.iota.cafe:443");
    client.set_send_options(SendTrytesOptions::default());

    //Create the Channels Author instance from the seed and the client
    let mut author = Author::new(&seed, "utf-8", PAYLOAD_BYTES, false, client);

    //Get the channel address from our newly created Author
    let channel_address = author.channel_address().unwrap().to_string();

    //Send the announce message to the Tangle, this shares the signature Keys of the Author
    let announcement_id = send_announce_message(&mut author).unwrap();

    //Generate the Message-Link of the announce message from the channel address and the message-id
    let announce_link = Address::from_str(&channel_address, &announcement_id).unwrap();
    println!("Announce message sent! ");

    //Print out the channel id, it is composed of Channel Address and the Announcement id
    let channel_id = format!("{}:{}", channel_address, announcement_id);
    println!("\n Channel Id : \n {} \n ", channel_id);

    let mut last_message_link = announce_link;

    //Send 5 messages to the channel, each time linking the new messages to the one before
    for i in 0..5 {
        let message_to_send = format!("Message nr: {}", i);
        let message_id = send_public_msg(&message_to_send, last_message_link, &mut author).unwrap();

        //get new message link for next message
        last_message_link = Address::from_str(&channel_address, &message_id).unwrap();
        println!("Sent: {}", message_to_send);
    }

    //Print out the command to run the subscriber with the given channel id
    println!(
        "To read the messages out of the channel copy and run this: \n \n \t cargo run --bin subscriber {} \n \n ",
        channel_id
    );
}
