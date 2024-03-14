/**************************************************************************
  Electrium Mobility Bakfiets display
  Contributers: 
 **************************************************************************/

#include <SPI.h>
#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

#define SCREEN_WIDTH 128
#define SCREEN_HEIGHT 64
#define SDA_PIN 10
#define SCL_PIN 9


// Declaration for an SSD1306 display connected to I2C (SDA, SCL pins)
#define OLED_RESET     -1 // Reset pin # (or -1 if sharing Arduino reset pin)
#define SCREEN_ADDRESS 0x3C ///< See datasheet for Address; 0x3D for 128x64, 0x3C for 128x32
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);

#define NUMFLAKES     10 // Number of snowflakes in the animation example

#define LOGO_HEIGHT   16
#define LOGO_WIDTH    16
static const unsigned char PROGMEM logo_bmp[] =
{ 0b00000000, 0b11000000,
  0b00000001, 0b11000000,
  0b00000001, 0b11000000,
  0b00000011, 0b11100000,
  0b11110011, 0b11100000,
  0b11111110, 0b11111000,
  0b01111110, 0b11111111,
  0b00110011, 0b10011111,
  0b00011111, 0b11111100,
  0b00001101, 0b01110000,
  0b00011011, 0b10100000,
  0b00111111, 0b11100000,
  0b00111111, 0b11110000,
  0b01111100, 0b11110000,
  0b01110000, 0b01110000,
  0b00000000, 0b00110000 };

void setup() {
  Serial.begin(9600);
  Wire.begin(SDA_PIN, SCL_PIN);

  // SSD1306_SWITCHCAPVCC = generate display voltage from 3.3V internally
  if(!display.begin(SSD1306_SWITCHCAPVCC, SCREEN_ADDRESS)) {
    Serial.println(F("SSD1306 allocation failed"));
    for(;;); // Don't proceed, loop forever
  }

  // Clear the buffer
  display.clearDisplay();

  loop();

}

void drawBattery(uint8_t percentage) {
  int16_t batteryWidth = 48; 
  int16_t batteryHeight = 20;
  int16_t batteryX = SCREEN_WIDTH - batteryWidth - 75; // Position to the right
  int16_t batteryY = 5; // Position at the top

  // Calculate the width of the battery's fill
  int16_t fillWidth = (batteryWidth - 4) * percentage / 100; 

  // battery outline
  display.drawRect(batteryX, batteryY, batteryWidth, batteryHeight, SSD1306_WHITE);
  display.fillRect(batteryX + batteryWidth, batteryY + 3, 2, batteryHeight - 6, SSD1306_WHITE);

  // battery fill
  display.fillRect(batteryX + 2, batteryY + 2, fillWidth, batteryHeight - 4, SSD1306_WHITE);

  // battery percent
  display.setTextSize(2);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(batteryX + batteryWidth + 6, batteryY + 3);
  display.print(percentage);
  display.print('%');
}


void loop() {
  display.clearDisplay(); 
  // Battery display
  uint8_t batteryLevel = 50;// hard coded for now but will need to figure out how to get the actual battery %
  drawBattery(batteryLevel);

  // Speed

  // Speed Units

  // PA divider
  display.drawLine(90, 10, 90, display.height()-11, SSD1306_WHITE);

  // PA text
  display.setTextSize(2);
  display.setTextColor(SSD1306_WHITE);
  display.setCursor(100, 10);
  display.println(F("PA"));
  display.display(); 

  // PA level
  
  


  // for(int16_t i=0; i<display.width(); i+=4) {
  //   display.drawLine(0, 0, i, display.height()-1, SSD1306_WHITE);
  //   display.display(); // Update screen with each newly-drawn line
  //   delay(1);
  // }
}

