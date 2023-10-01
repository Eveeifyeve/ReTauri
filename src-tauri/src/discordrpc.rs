extern crate discord_rpc_client;

use std::{env, thread, time};
use discord_rpc_client::Client;

fn main() {
    // Get our main status message
    let state_message = env::args().nth(1).expect("Requires at least one argument");

    // Create the client
    let mut drpc = Client::new(425407036495495169);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Set the activity
    drpc.set_activity(|act| act.state(state_message))
        .expect("Failed to set activity");

    // Wait 10 seconds before exiting
    thread::sleep(time::Duration::from_secs(10));
}