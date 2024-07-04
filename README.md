Our team is building an electric Bakfiets, a Dutch two-wheeled cargo bike. It has a large cargo box in the front to permit the transport of heavy items, such as groceries, and as such provides an environmentally-friendly alternative to cars.

**Mechanical:**
Our vehicle features a custom-built AISI 4130 frame that was prepared for welding in an MDF jig.

**Electrical:**

**Firmware:**
Our bike features a custom display and interface powered by an ESP32. In order to integrate our Bafang BBS02 motor, which has a built-on motor controller, we reverse-engineered the motor communication protocol to be able to display data to our custom display.

To run the embedded display, upload the main.cpp file onto the ESP32
