use rust_gpiozero::*;
use std::{thread, time};

fn main() {

    let one_sec = time::Duration::from_secs(1);

    // declare GPIO #23 on the BCM
    let bcm_gpio: u8 = 23;
    // create the GPIO object
    let mut output = OutputDevice::new(bcm_gpio);
    // read the pin for confirmation
    println!("GPIO pin : {}", output.pin());
    // set the 1 level
    output.set_active_high(true);
    // read the active high
    println!("GPIO active high : {}", output.active_high());

    loop {
        output.on();
        println!("GPIO ON (state: {})", output.value());

        thread::sleep(one_sec);

        output.off();
        println!("GPIO ON (state: {})", output.value());

        thread::sleep(one_sec);
    }
}
