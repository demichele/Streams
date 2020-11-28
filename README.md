## Introduction

The Folder structure of the Example:  
  
```  
Cargo.toml                  // rust package and dependency file  

bin/  
  - author.rs               // code to run the author  
  - subscriber.rs           // code to run the subscriber  

src/   
 - author_actions/          // folder that contains the functions to execute the author actions  
   - announce.rs            // function to send the announce message  
   - send_public_msg.rs     // function to send a public message to the channel  
   - mod.rs                 // module file   
     
 - subscriber_actions/      // folder that contains the functions to execute the subscriber actions 
   - connect_to_channel.rs  // function to connect the subscriber to a channel
   - read_message.rs        // function to read a message from the channel
   - mod.rs                 // module file 

 - lib.rs                   // library file containing the random_seed() helper function
```

## Running the Example  

First you will have to run the Author, this will create a new channel, announce the authors public key and send a few messages into the channel.  
To run the Author use:  
`cargo run --bin author`   
  
This will print out somthing like:  
``` 
Announce message sent! 

 Channel Id : 
 5cf3287e3ea1046a5604d25214f9ed99c6db7e45af6a318445bb43ba896d27f90000000000000000:febf5da4450c378249fd3801 
 
Sent: Message nr: 0
Sent: Message nr: 1
Sent: Message nr: 2
Sent: Message nr: 3
Sent: Message nr: 4
To read the messages out of the channel copy and run this: 
 
         cargo run --bin subscriber 5cf3287e3ea1046a5604d25214f9ed99c6db7e45af6a318445bb43ba896d27f90000000000000000:febf5da4450c378249fd3801  
```  


Now you can run the subscriber by copy/pasting the command printed above, this will use the channel_id to connect to the channel and derive all subsequent message ids.  
To run the subscriber use (replace `your_channel_id` with the output of the author):  
`cargo run --bin subscriber <your_channel_id>`  
  
This will print out something like:  
```
Connected to channel! 
 Looking for messages ...
Found 5 messages in the Channel!
Decoded message: "Message nr: 0"
Decoded message: "Message nr: 1"
Decoded message: "Message nr: 2"
Decoded message: "Message nr: 3"
Decoded message: "Message nr: 4"
```

Great you now succesfully wrote data into the Tangle through the IOTA-Streams Channels, and got it back out!