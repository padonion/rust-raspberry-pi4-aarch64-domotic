# Simple domotic project using Rust language on Raspberry Pi4-B using an ubuntu 20.04 aarch64

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
