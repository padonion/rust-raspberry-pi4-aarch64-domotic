# Simple domotic project using Rust language on Raspberry Pi4-B using an ubuntu 20.04 aarch64

## Goal

I created this project to fix a small problem I have in a remote area.

**The problem**: In this remote area, I have a modem which allows the IoT to communicate some data on internet. Unfortunately the power in this remote area is not really stable, and time to time a micro shutdown of the power happens. If this powerdown is short enough, it leaves the modem in an unknown state and it will never recover from the shutdown.

I can hear some of you saying: "Why don't you put a UPS?". Well that's a good idea, but I was looking at a more versatile solution and a fun project.

**The solution**: I decided to go smarter than just a UPS and look for an "any disconnect event" solution which would prevent internet disconnection from power shutdown but also modem bug and lock state.

I decided to move forward with a Raspberry Pi 4 acting as an internet watchdog and powering off and on the modem by itself if internet is lost.

## Configuration

**The material**:

- [Raspberry Pi 4B/4GB](https://www.digikey.com/product-detail/en/raspberry-pi/RASPBERRY-PI-4B-4GB/1690-RASPBERRYPI4B-4GB-ND/10258781)
- [16GB SD Card](https://www.digikey.com/product-detail/en/panasonic-electronic-components/RP-SMLE16DA1/P17029-ND/5119436)
- [USB-C Power supply](https://www.digikey.com/product-detail/en/raspberry-pi/RPI-USB-C-POWER-SUPPLY-BLACK-US/1690-RPIUSB-CPOWERSUPPLYBLACKUS-ND/10258759)
- [Heat Sink **Do not forget it!**](https://www.digikey.com/product-detail/en/seeed-technology-co-ltd/110991329/1597-110991329-ND/10451888)
- [Micro relay 110V-220V](https://www.digikey.com/product-detail/en/adafruit-industries-llc/4409/1528-4409-ND/10669532)
- [Cable for micro-relay](https://www.digikey.com/product-detail/en/adafruit-industries-llc/3894/1528-2697-ND/9603620)
- [Raspberry Pi 4 case](https://www.digikey.com/product-detail/en/raspberry-pi/RASPBERRY-PI-4-CASE-RED-WHITE/1690-RASPBERRYPI4CASERED-WHITE-ND/10258761)

I give a link to digikey as a reference to the material I bought (the case is optional, I don't know yet how I will mount all that together).

Our bench now:

![Bench](images/bench.jpg)

**The Operating System**:

The Raspberry Pi4 is a 64bits ARM architecture. At this time, ```raspbian``` is limited to 32bits. One of the reason is because Broadcom is not not providing the 64bits version of the video core. But as we are not using the video output of our raspberry, I would say that I do not care about this part. Therefore I decided to move forward with the [Ubuntu 20.04 aarch64 available on the Ubuntu website](https://ubuntu.com/download/raspberry-pi/thank-you?version=20.04&architecture=arm64+raspi).

There is nothing to do, except downloading the image, and following their tutorial to install it on a SD card.

```ssh``` is installed and activated on boot, I did not need any screen on my Raspberry at all.

```uname -a``` on a ```ssh``` session shows us that we are on a 64 bits ARM operating system:
```
ubuntu@ubuntu:~$ uname -a
Linux ubuntu 5.4.0-1008-raspi #8-Ubuntu SMP Wed Apr 8 11:13:06 UTC 2020 aarch64 aarch64 aarch64 GNU/Linux
```
```file /usr/bin/yes``` confirms that binaries are 64bits:
```sh
ubuntu@ubuntu:~$ file /usr/bin/yes
/usr/bin/yes: ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=953226318f1646263799b28ffc77e0e9e2d88baf, for GNU/Linux 3.7.0, stripped
```

**Set ```gpio``` group and permission**:

We need to add ```gpio``` group to the ubuntu and the permissions to the gpio devices to make our life easier (we could run all our apps but that would not be very serious).

First, add ```gpio``` group and add user ```ubuntu``` to it:
```sh
sudo groupadd -f -r gpio
sudo adduser ubuntu gpio
```
Second, we need to create a ```udev``` rule to set this group to the devices by creating ```/etc/udev/rules.d/99-gpio.rules```:
```sh
ubuntu@ubuntu:~$ cat /etc/udev/rules.d/99-gpio.rules 
SUBSYSTEM=="bcm2835-gpiomem", KERNEL=="gpiomem", GROUP="gpio", MODE="0660"
SUBSYSTEM=="gpio", KERNEL=="gpiochip*", GROUP="gpio", MODE="0660"
SUBSYSTEM=="gpio", KERNEL=="gpiochip*", ACTION=="add", PROGRAM="/bin/sh -c 'chown root:gpio /sys/class/gpio/export /sys/class/gpio/unexport ; chmod 220 /sys/class/gpio/export /sys/class/gpio/unexport'"
SUBSYSTEM=="gpio", KERNEL=="gpio*", ACTION=="add", PROGRAM="/bin/sh -c 'chown root:gpio /sys%p/active_low /sys%p/direction /sys%p/edge /sys%p/value ; chmod 660 /sys%p/active_low /sys%p/direction /sys%p/edge /sys%p/value'"
```

**Activate the firewall**:

Yes, our raspberry will be in the world and need some protection.
> Do not forget to add the ssh authorisation before activating the firewall, otherwise you will loose connection and all possibility to remote-connect!
```sh
sudo ufw allow ssh
sudo ufw enable
```

## Installation and configuration of 'rust' on our development platform

In all my examples, I will use my laptop as development pltaform and send only the binaries to the Raspberry PI (I want my raspberry to be clean, not as a dev platform).

I will use ```rust``` as development language. Why ? Because:

- it is fun to learn a new language
- it has an interesting concept of memory management and is very secure
- it has low level capability
- [finally I let you read all the reasons there](https://www.rust-lang.org/).

To install it on your laptop folling the installation instruction: [Install Rust](https://www.rust-lang.org/tools/install)

Now we need to add the cross-compilation for ```aarch64```. For linux (Ubuntu to be more precise) you have to install first the compiler tools with this command line (I let you figure out how to install aarch64 compiler on your OS):
```sh
sudo apt install gcc-aarch64-linux-gnu
```

Then you have to install ```rust``` target with the following command line:
```sh
rustup target add aarch64-unknown-linux-gnu
```

And finally you have to tell ```cargo``` what to use for the target in the file ```~/.cargo/config```:

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

> if the file ```~/.cargo/config``` does not exist yet, create it.

This is it. Your development platform is ready.

Lets go for an example, create a ```hello-world``` wherever you want with the following line:

```
cargo new --bin hello-world
```

Move in this project's folder and compile it for aarch64:

```
cd hello-world
cargo build --target aarch64-unknown-linux-gnu
```

Lets verify that the binary is compiled as we want:
```bash
padonion@padonion$file target/aarch64-unknown-linux-gnu/debug/hello-world
target/aarch64-unknown-linux-gnu/debug/hello-world: ELF 64-bit LSB shared object, ARM aarch64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-linux-aarch64.so.1, BuildID[sha1]=b67a7f69372f533a85b329095443f4dccb192a81, for GNU/Linux 3.7.0, with debug_info, not stripped
```

Now we have to send this file to our Raspberry with ssh copy:
```
scp target/aarch64-unknown-linux-gnu/debug/hello-world ubuntu@XXX.XXX.XXX.XXX:~
```

And inside a ```ssh``` session on our Raspberry we can execute it:
```bash
ubuntu@ubuntu:~$ ./hello-world 
Hello, world!
ubuntu@ubuntu:~$ 
```

It works!
