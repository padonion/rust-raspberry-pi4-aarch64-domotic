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
