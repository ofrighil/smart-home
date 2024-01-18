# Hacker News Feed

This project uses the [Adafrut Martix Portal (M4)](https://www.adafruit.com/product/4745) along with a [64x32 RGB LED Matrix](https://www.adafruit.com/product/2278) to display a feed from Hacker News using the [Hacker News API](https://github.com/HackerNews/API).
Inspired by the [RGB Matrix NYT Text Scroller Code](https://learn.adafruit.com/rgb-matix-nyt-text-scroller/rgb-matrix-nyt-text-scroller-code).

## Requirements
This code was written writh CircuitPython 8.2.9. The following libraries are required (and should be put in `lib`):
* adafruit_bitmap_font
* adafruit_display_text
* adafruit_io
* adafruit_matrixportal
* adafruit_minimqtt
* adafruit_fakerequests.mpy

Additionally, there should be a `secrets.py` file:
```python
    'ssid': '<network name>'
    'password': '<network password>'
```
This will enable the internet connection to pull the data.
