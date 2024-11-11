# TWAI/CAN Sender Code
ESP32 code for the CAN sender.

## Setup
If ESP-IDF is not installed, follow [espressif's installation guide.](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html)


If ESP-IDF is installed, run
```
. $HOME/esp/esp-idf/export.sh
cd <project-directory>
idf.py set-target esp32
idf.py build
idf.py -p <PORT> flash monitor
```
