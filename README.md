## Simple domotic project using Rust language on Raspberry Pi4-B using an ubuntu 20.04 aarch64

I created this project to fix a small problem I have in a remote area.

**The problem**: In this remote area, I have a modem which allows the IoT to communicate some data on internet. Unfortunately the power in this remote area is not really stable, and time to time a micro shutdown of the power happens. If this powerdown is short enough, it leaves the modem in an unknown state and it will never recover from the shutdown.

I can hear some of you saying: "Why don't you put a UPS?". Well that's a good idea, but I was looking at a more versatile solution and a fun project.

**The solution**: I decided to go smarter than just a UPS and look for an "any disconnect event" solution which would prevent internet disconnection from power shutdown but also modem bug and lock state.

I decided to move forward with a Raspberry Pi 4 acting as an internet watchdog and powering off and on the modem by itself if internet is lost.

[The Wiki will help you in](https://github.com/padonion/rust-raspberry-pi4-aarch64-domotic/wiki):

1. installing Ubuntu 20.04 ARM 64bits on the Raspebrry PI 4
2. installing Rust on your development platform
3. testing access to the GPIO in Rust

## `gpio-test` example

First example will blink the relay 4 times.

```
cargo build --target aarch64-unknown-linux-gnu --example gpio-test
```

and copy the binary for test on your raspberry

```
scp target/aarch64-unknown-linux-gnu/debug/examples/gpio-test ubuntu@XXX.XXX.XXX.XXX:~
```

and then `ssh` into your raspberry and execute it

```
./gpio-test
```

## `ping` example

Second example will ping a server on internet. I chose 8.8.8.8 because it is Google's DNS and I know it.

```
cargo build --target aarch64-unknown-linux-gnu --example ping-test
```

and copy the binary for test on your raspberry

```
scp target/aarch64-unknown-linux-gnu/debug/examples/ping-test ubuntu@XXX.XXX.XXX.XXX:~
```

and then `ssh` into your raspberry and execute it

> carfting ICMP packets requires privileges. But this is not a problem because our final application will run as a service.

```
sudo ./ping-test
```

## final project
