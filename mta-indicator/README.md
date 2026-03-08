# MTA Indicator

This project uses a [Raspberry Pi 4 Model B](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/) along with a [64x32 RGB LED Matrix](https://www.adafruit.com/product/2278) (connected with a [2x20 Socket Riser Header for Raspberry Pi HATs and Bonnets](https://www.adafruit.com/product/4079) and [Adafruit RGB Matrix Bonnet for Raspberry Pi](https://www.adafruit.com/product/3211)) to show the incoming subway trains into a station in NYC.
Parameterized with station, service, and direction.

This app handles the drawing.
I wrote a separate library (see [pulse](https://github.com/ofrighil/pulse)) that does the querying.

## Requirements

See the `Cargo.toml` file.

## Changelog

### 2026-03-06

* Add most of the relevant files.