# GPS BLE RECEIVER for AGRO NAVIGATION APP

### AGRO GPS receiver for Parallel Driving App 
DIY receiver for agro direction system (or guidance system).

App description: https://gps-free.net/parallel-driving/

[<img width="273" alt="Screenshot 2024-05-22 at 17 55 38" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/04f9eece-f1e4-480b-ac1b-ad0384a98eca">](https://apps.apple.com/ru/app/agro-navigation/id1625258870)

[<img width="271" alt="Screenshot 2024-05-22 at 17 56 20" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/18e10f75-b2f4-4f41-9fa1-3fa07ac9100b">](https://play.google.com/store/apps/details?id=com.shlyagin.parallel_driving)

Another more simplest receiver - https://github.com/Udj13/Agro-Navigation-receiver

This GPS receiver is better than first one because it's operates at free L5 satellites for decimeter accuracy, but more difficult to assemble.

For decimeter accuracy TAU1201 module is used.
ESP32 controller with built-in bluetooth low energy (BLE).

You can build it yourself and use it with my parallel driving app.


### Components

- ESP32 module: ESP-WROOM-32

> [!IMPORTANT]
> ESP32, ESP32-C, ESP32-S, ... These are different modules, buy ESP32!

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

1. Connect ESP32 to computer (drivers: https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers)
2. Download Flash Download Tools (https://www.espressif.com/en/support/download/other-tools)
3. Download *.bin files from Releases (https://github.com/Udj13/gps-ble-receiver/releases/)
4. Start Flash Download Tools
   
![Снимок экрана 2024-05-17 151842](https://github.com/Udj13/gps-ble-receiver/assets/54446451/4a78e5be-3918-43f7-807b-19eedb5f1a60)

6. Select ChipType - ESP32
7. Make the settings as in the picture and press Start

![Screenshot 2024-05-17 180945](https://github.com/Udj13/gps-ble-receiver/assets/54446451/b5f6d922-0f72-49b0-ae6c-8d3f9247b141)


|file name            |address |
|---------------------|--------|
|bootloader.bin       |  0x1000|
|partition-table.bin  |  0x8000|
|gps-ble-receiver.bin | 0x10000|


BAUD: 921600 (or less but the firmware will take longer)
