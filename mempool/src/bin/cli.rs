use std::error::Error;
use async_std::io;
use futures::{prelude::*, select};
use mempool::app::App;
use libp2p::Multiaddr;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new().await;

    // Reach out to another node if specified
    if let Some(to_dial) = std::env::args().nth(1) {
        let address: Multiaddr = to_dial.parse().expect("User to provide valid address.");
        app.connect(address).await;
    }

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => {
                let cmd = line.expect("Stdin not to close");
                app.handle_command(cmd)
            },
            event = app.swarm.select_next_some() => app.handle_network_event(event)
        }
    }
}