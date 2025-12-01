# Hacker News Feed

This project uses the [Adafrut Martix Portal (M4)](https://www.adafruit.com/product/4745) along with a [64x32 RGB LED Matrix](https://www.adafruit.com/product/2278) to display a feed from Hacker News using the [Hacker News API](https://github.com/HackerNews/API).
Inspired by the [RGB Matrix NYT Text Scroller Code](https://learn.adafruit.com/rgb-matix-nyt-text-scroller/rgb-matrix-nyt-text-scroller-code).

## Requirements
The code is compatible with CircuitPython v10.0.3.
The following libraries are required (and should be put in `lib/`):
* adafruit_bitmap_font
* adafruit_display_text
* adafruit_imageload
* adafruit_io
* adafruit_matrixportal
* adafruit_minimqtt
* adafruit_fakerequests.mpy
* adafruit_ticks.mpy

These libraries can be found bundled with other libraries [here](https://circuitpython.org/libraries).

Additionally, there should be a `settings.toml` file:
```toml"
    CIRCUITPY_WIFI_SSID="<network name>"
    CIRCUITPY_WIFI_PASSWORD="<network password>"
```
This will enable the internet connection to pull the data.

## Changelog

### 2025-12-01

* Updated to use UF2 bootloader v3.16.0 and CircuitPython v10.0.3
* Due to the major version upgrade to CircuitPython v10.0.3, upgraded the existing libraries and also added a couple additional libraries:
    * adafruit_imageload
    * adafruit_ticks.mpy

### 2024-01-17

* First added using CircuitPython v8.2.9 (forgot to record which UF2 bootloader version was used)
* Libraries used:
    * adafruit_bitmap_font
    * adafruit_display_text
    * adafruit_io
    * adafruit_matrixportal
    * adafruit_minimqtt
    * adafruit_fakerequests.mpy