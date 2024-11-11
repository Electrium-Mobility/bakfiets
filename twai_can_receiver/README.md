# TWAI/CAN Receiver Code
ESP32 code for the CAN receiver. Also connects to a display using I2C and draws graphics with lvgl.

## Setup
If ESP-IDF is not installed, follow [espressif's installation guide.](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/index.html)


If ESP-IDF is installed, run
```
. $HOME/esp/esp-idf/export.sh
cd <project-directory>
idf.py set-target esp32
idf.py add-dependency "espressif/esp_lvgl_port^2.3.0"
idf.py menuconfig
```
In menuconfig,
 Component config --> LVGL Configuration --> Font usage --> Enable built-in fonts  --> Enable Montserrat 32
this will enable the larger font size. Then click S to save and Q to exit menuconfig. 
Then run,
```
idf.py build
idf.py -p <PORT> flash monitor
```

