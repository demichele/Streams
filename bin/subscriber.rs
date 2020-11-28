// import the IOTA-Streams Dependencies
use iota_streams::app::transport::tangle::{client::Client as IotaClient, PAYLOAD_BYTES};
use iota_streams::app_channels::api::tangle::{Address, Subscriber};

//Import the standard env tools to read command-line arguments
use std::env;

//Import our code for the Subscriber
use example::{
    random_seed,
    subscriber_actions::{connect_to_channel::connect, read_message::read_message},
};

fn main() {
    // Read the Channel Id from the command line
    let args: Vec<String> = env::args().collect();
    let channel_id = &args[1];

    // Split the Channel Id into channel address and announcement id
    let str_iter = channel_id.split(":").collect::<Vec<&str>>();
    let channel_address = str_iter[0];
    let announce_id = str_iter[1];

    // Generate the link to the annuncement message in the channel
    let announcement_link = Address::from_str(&channel_address, &announce_id).unwrap();

    // Generate a new random seed
    let seed = random_seed();

    // Create an IOTA client instance, this is our connection to the IOTA-Network
    let client: IotaClient = IotaClient::new_from_url("https://nodes.iota.cafe:443");

    // Create the Channels Subscriber instance from the seed and the client
    let mut subscriber = Subscriber::new(&seed, "utf-8", PAYLOAD_BYTES, client);

    // Connect to the Channel by finding and digesting the announce message from the it's link
    connect(&mut subscriber, announcement_link).unwrap();
    println!("Connected to channel! \n Looking for messages ...");

    // Get a list of the message ids of all messages in the Channel
    let id_list = get_all_msg_ids(&mut subscriber).unwrap();
    println!("Found {:?} messages in the Channel!", id_list.len());

    // Read out each message by creating it's link from the channel address and the message id
    for signed_message_id in id_list {
        // Generate the link
        let message_link = Address::from_str(&channel_address, &signed_message_id).unwrap();

        //Read the message
        let message = read_message(&mut subscriber, message_link).unwrap();
        println!("Decoded message: {:?}", message);
    }
}

// Gets the message Id for each message in the channel
fn get_all_msg_ids(subscriber: &mut Subscriber<IotaClient>) -> Option<Vec<String>> {
    let mut ids: Vec<String> = vec![];

    //Messages are returned in lists, here we get the id of the first one
    let mut msgs = subscriber.fetch_next_msgs();
    ids.push(msgs[0].link.msgid.to_string());

    //continue reading out messages untill there are no more, and an empty list is returned
    while !msgs.is_empty() {
        msgs = subscriber.fetch_next_msgs();

        //here we iterate as the list might be empty and the [0] element wouldn't exist
        for msg in &msgs {
            ids.push(msg.link.msgid.to_string());
        }
    }
    Some(ids)
}
