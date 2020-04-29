use rust_gpiozero::*;
use std::{thread, time};

fn main() {

    // define a one second time duration
    let one_sec = time::Duration::from_secs(1);

    // declare GPIO #23 on the BCM (where we plug our relay)
    const BCM_GPIO_RELAY: u8 = 23;

    // create the GPIO object (mutable as we want to change the output)
    let mut output = OutputDevice::new(BCM_GPIO_RELAY);
    
    // read the pin for confirmation
    println!("GPIO pin : {}", output.pin());

    // set the true level to be high level on the pin (3.3V)
    output.set_active_high(true);

    // read the active high
    println!("GPIO active high : {}", output.active_high());

    // we do it 4 times
    for _ in 0..4 {

        // we activate the relay
        output.on();
        println!("GPIO ON (state: {})", output.value());

        // we wait for 1s
        thread::sleep(one_sec);

        // we deactivate the relay
        output.off();
        println!("GPIO ON (state: {})", output.value());

        // we wait for 1s
        thread::sleep(one_sec);
    }
}
