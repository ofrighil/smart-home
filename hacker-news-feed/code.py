import board
import terminalio

from adafruit_display_text.label import Label
from adafruit_matrixportal.matrixportal import MatrixPortal


ABBREVIATIONS = {
    "topstories": "top",
    "newstories": "new",
    "beststories": "best",
    "askstories": "ask",
    "showstories": "show",
    "jobstories": "job",
}
ENDPOINT = "topstories"

BASE_URL = "https://hacker-news.firebaseio.com/v0"

matrixportal = MatrixPortal(status_neopixel=board.NEOPIXEL, debug=False)
display = matrixportal.display
network = matrixportal.network

height = display.height
width = display.width

font = terminalio.FONT
font_width, font_height = font.get_bounding_box()

def center_width(width, text_length, font_width):
    return (width - (font_width * text_length)) // 2

startup = Label(font=font, text="connecting", color=0x0099FF)
startup.x = center_width(width, len(startup.text), font_width)
startup.y = height // 2

matrix_group = display.root_group
while not network._wifi.is_connected:
    display.root_group = startup
    network.connect()
display.root_group = matrix_group

matrixportal.add_text(
    text_font=font,
    text_position=(1, height // 8 + 2),
    text="HN",
    text_color=0xFF6600,
)
matrixportal.add_text(
    text_font=font,
    text_position=(6 * 2 + 1, height // 8 + 2),
    text=ABBREVIATIONS[ENDPOINT],
    text_color=0xFFFFFF,
)

body = matrixportal.add_text(
    text_font=font,
    text_position=(1, 4 * (height // 8 + 1)),
    text_color=0x0099FF,
    scrolling=True,
)

loading = matrixportal.add_text(
    text_font=font,
    text_position=(1, 4 * (height // 8 + 1)),
    text_color=0x0099FF,
)

while True:
    matrixportal.set_text("Loading...", loading)
    story_ids = network.fetch(f"{BASE_URL}/{ENDPOINT}.json")
    if story_ids.status_code == 200:
        matrixportal.set_text("", loading)
        for story_id in story_ids.json()[:25]:
            try:
                matrixportal.scrolling = True
                result = network.fetch_data(f"{BASE_URL}/item/{story_id}.json", json_path=("title",))[0]
                matrixportal.set_text(result, body)
                matrixportal.scroll_text(frame_delay=0.025)
            except:
                continue
    else:
        continue
