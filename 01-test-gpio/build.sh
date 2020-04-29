cargo build --target aarch64-unknown-linux-gnu
scp target/aarch64-unknown-linux-gnu/debug/rp4-gpio-test ubuntu@192.168.1.29:~
