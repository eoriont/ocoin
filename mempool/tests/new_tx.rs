// use blockchain::{signed_transaction::SignedTransaction, transaction::Transaction};
// use futures::prelude::*;
// use mempool::app::App;
// use std::error::Error;

// macro_rules! net_evt {
//     ($app:ident) => {
//         let event = $app.swarm.select_next_some().await;
//         $app.handle_network_event(event);
//     };
// }

// #[async_std::test]
// async fn test_new_transaction() -> Result<(), Box<dyn Error>> {
//     let mut app1 = App::new().await;
//     let mut app2 = App::new().await;

//     // Local ip, global ip
//     net_evt!(app1);
//     net_evt!(app1);
//     net_evt!(app2);
//     net_evt!(app2);
//     // let addr = app1.addresses.get(0).unwrap().clone();
//     // app2.connect(addr).await;

//     // Connect
//     // TODO: Dialing doesn't work here for some reason
//     // match app2.swarm.dial(app1.swarm.local_peer_id().clone()) {
//     //     Ok(_) => println!("Dialed"),
//     //     Err(x) => println!("Err: {}", x),
//     // }
//     match app2.swarm.dial(app1.addresses.get(0).unwrap().clone()) {
//         Ok(_) => println!("Dialed"),
//         Err(x) => println!("Err: {}", x),
//     }

//     app2.new_transaction(SignedTransaction {
//         transaction: Transaction::new("Eli".to_string(), "Ari".to_string(), 10.0),
//         signature: "Hello".to_string(),
//     });

//     // Local ip, global ip, message
//     // net_evt!(app1);

//     println!("{:?}", app1.mempool.transactions);
//     assert_eq!(app1.mempool.transactions.len(), 1);

//     Ok(())
// }
