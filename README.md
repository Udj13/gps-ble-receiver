# GPS BLE RECEIVER for AGRO NAVIGATION APP

### AGRO GPS receiver for Parallel Driving App 
DIY receiver for agro direction system or guidance system.

App description: https://gps-free.net/parallel-driving/

![image](https://github.com/Udj13/gps-ble-receiver/assets/54446451/524e261a-99ae-4022-a88f-6ca315bb27bc)

https://play.google.com/store/apps/details?id=com.shlyagin.parallel_driving

![image](https://github.com/Udj13/gps-ble-receiver/assets/54446451/a71dd02e-04be-40b1-a73b-5265b574e458)

https://apps.apple.com/ru/app/agro-navigation/id1625258870

Another more simple receiver - https://github.com/Udj13/Agro-Navigation-receiver


This GPS receiver operates at free L5 satellites for decimeter accuracy.
TAU1201 module is used.

You can build it yourself and use it with my parallel driving app.


### Components

- ESP32 module: ESP-WROOM-32
- GPS L1 & L5 module: ALLYSTAR TAU1201
- GPS antenna L1 & L5 ranges (1575.42MHz & 1176.42MHz)

### Wiring

Just connect 4 wires and connect USB

- GPS 5v pin -> ESP32 VIN pin
- GPS GNG pin -> ESP32 GND pin
- GPS TX pin -> ESP32 D17 pin
- GPS RX pin -> ESP32 D16 pin

### Module firmware

Releases: https://github.com/Udj13/gps-ble-receiver/releases/

### How to flash module firmware
