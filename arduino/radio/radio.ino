const int OUTPUT_PIN = 9;   // Pin to output the 62.7 kHz PWM signal
const int HIGH_FREQ = 60000;
const int LOW_FREQ = 8571;

unsigned long lastActivityTime = 0;  // Store the last time activity was detected
const unsigned long timeoutInterval = 60000;  // 1 minute timeout (60,000 ms)

void setup() {


  // Start serial communication at 115200 baud rate
  Serial.begin(115200);

  // Wait for serial monitor to connect (for boards with native USB like Leonardo)
  while (!Serial) {
    ;
  }
}

void loop() {
  // Check if data is available to read from serial
  if (Serial.available() > 0) {
    // Reset the timeout timer when a character is received
    lastActivityTime = millis();
    
    // Read the incoming character (not used here, but can be used for debugging)
    char incomingChar = Serial.read();

    if (incomingChar == 'H')
    {
      tone(OUTPUT_PIN, HIGH_FREQ);
    }

    if (incomingChar == 'L')
    {
      tone(OUTPUT_PIN, LOW_FREQ);
    }
  }

  // Check if more than 1 minute has passed without any serial activity
  if (millis() - lastActivityTime > timeoutInterval)
  {
    noTone(OUTPUT_PIN);
  }
}