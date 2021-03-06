use std::{thread, time};

use rppal::gpio::Gpio;

fn main() {
    // define a one second time duration
    let one_sec = time::Duration::from_secs(1);

    // declare GPIO #23 on the BCM (where we plug our relay)
    const BCM_GPIO_RELAY: u8 = 23;

    // create the GPIO object (mutable as we want to change the output)
    let gpios = match Gpio::new() {
        Ok(gpios) => gpios,
        Err(msg) => panic!("Error: {}", msg),
    };

    // retrieve the GPIO pin as an Output
    let mut output = match gpios.get(BCM_GPIO_RELAY) {
        Ok(output) => output.into_output(),
        Err(msg) => panic!("Error: {}", msg),
    };

    // read the pin for confirmation
    println!("GPIO pin : {}", output.pin());

    // we do it 4 times
    for _ in 0..4 {
        // we activate the relay
        output.set_high();
        println!("GPIO #{} HIGH (state: {})", BCM_GPIO_RELAY, output.is_set_high());

        // we wait for 1s
        thread::sleep(one_sec);

        // we deactivate the relay
        output.set_low();
        println!("GPIO #{} LOW (state: {})", BCM_GPIO_RELAY, output.is_set_high());

        // we wait for 1s
        thread::sleep(one_sec);
    }
}
