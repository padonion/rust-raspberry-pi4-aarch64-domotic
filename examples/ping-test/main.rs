// add fastping_rs crate for ping
use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};

fn main() {

    // lets create 2 unmutable variables:
    // - pinger
    // - results which act as a receiver of pinger
    let (pinger, results) = match Pinger::new(None, None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e)
    };
    
    // set the destination address for ping
    pinger.add_ipaddr("8.8.8.8");

    // send a ping
    pinger.ping_once();

    // wait for a reply (or timeout)
    match results.recv() {
        Ok(result) => {
            // check for the response
            match result {
                Idle{addr} => {
                    println!("Address {} did not reply!", addr);
                },
                Receive{addr, rtt} => {
                    println!("Received reply from {} in {:?}.", addr, rtt);
                }
            }
        },
        // this should not happen as we do not use it async
        Err(_) => panic!("Error from result.recv() !!")
    }
}