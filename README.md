# GPS BLE RECEIVER for AGRO NAVIGATION APP

## AGRO DIY opensource hardware for parallel drivind

### Agro Navigation App for parallel driving 

This is an agro direction system (or guidance system).

App description: https://gps-free.net/parallel-driving/

[<img width="273" alt="Screenshot 2024-05-22 at 17 55 38" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/04f9eece-f1e4-480b-ac1b-ad0384a98eca">](https://apps.apple.com/ru/app/agro-navigation/id1625258870)

[<img width="271" alt="Screenshot 2024-05-22 at 17 56 20" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/18e10f75-b2f4-4f41-9fa1-3fa07ac9100b">](https://play.google.com/store/apps/details?id=com.shlyagin.parallel_driving)

If you don't have enough GPS accuracy in your phone, you can assemble and connect the easiest external GPS receiver.

The main idea of the receiver is to filter the flow of RMC and GGA sentences from NMEA stream and send them via bluetooth (BLE) in the application

This receiver can be used for other applications too, example code for using it: https://github.com/Udj13/test_nmea_reading/

If you have any questions, write to me by email shlyagin@gmail.com

### GPS L5 receiver description

Another more simplest receiver with standart GPS accuracy - https://github.com/Udj13/Agro-Navigation-receiver

This GPS receiver is better than the first one because it operates at free L5 satellites for decimeter accuracy, but it is more difficult to assemble.

For decimeter accuracy, the TAU1201 module is used.
ESP32 controller with built-in bluetooth low energy (BLE).

You can build it yourself and use it with my parallel driving app.


### Components

<img width="781" alt="Screenshot 2024-05-22 at 18 40 43" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/64669c92-3fe4-4335-bfa6-42a565609366">


- ESP32 module: ESP-WROOM-32

> [!IMPORTANT]
> ESP32, ESP32-C, ESP32-S, ... These are different modules, buy ESP32!


<img width="807" alt="Screenshot 2024-05-22 at 18 41 23" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/748a0c2a-8d65-485a-83e4-9e80b66cc7f7">

- GPS L1 & L5 module: ALLYSTAR TAU1201

<img width="630" alt="Screenshot 2024-05-22 at 18 41 44" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/d6298cd8-aa34-4037-9ace-155bf0072c55">


- GPS antenna L1 & L5 ranges (1575.42MHz & 1176.42MHz)

### Wiring

<img width="843" alt="Screenshot 2024-05-22 at 18 39 54" src="https://github.com/Udj13/gps-ble-receiver/assets/54446451/501e020e-88ed-4628-8cef-05caac05e644">


Just connect 4 wires and connect USB:

- GPS 5v pin -> ESP32 VIN pin
- GPS GNG pin -> ESP32 GND pin
- GPS TX pin -> ESP32 D17 pin
- GPS RX pin -> ESP32 D16 pin

Congratulations, your device is ready! Just need to update the firmware.

### Module firmware

Releases: https://github.com/Udj13/gps-ble-receiver/releases/

### How to flash module firmware

1. Connect ESP32 to computer (drivers: https://www.silabs.com/developers/usb-to-uart-bridge-vcp-drivers)
2. Download Flash Download Tools (https://www.espressif.com/en/support/download/other-tools)
3. Download *.bin files from Releases (https://github.com/Udj13/gps-ble-receiver/releases/)
4. Start Flash Download Tools
   
![Снимок экрана 2024-05-17 151842](https://github.com/Udj13/gps-ble-receiver/assets/54446451/4a78e5be-3918-43f7-807b-19eedb5f1a60)

6. Select ChipType - ESP32
7. Make the settings as in the picture and press Start. Sometimes you need to press "Boot" button on the board to start the firmware.

![Screenshot 2024-05-17 180945](https://github.com/Udj13/gps-ble-receiver/assets/54446451/b5f6d922-0f72-49b0-ae6c-8d3f9247b141)


|file name            |address |
|---------------------|--------|
|bootloader.bin       |  0x1000|
|partition-table.bin  |  0x8000|
|gps-ble-receiver.bin | 0x10000|


BAUD: 921600 (or less but the firmware will take longer)

### How to flash merged image firmware


|file name            |address |
|---------------------|--------|
|merged-img.bin       |  0x00|


![image](https://github.com/user-attachments/assets/4c9273b0-8249-42f3-bfb5-ff9a202630f8)



<img width="715" alt="Screenshot 2025-03-18 at 17 52 28" src="https://github.com/user-attachments/assets/13cfd0c0-3b84-4049-a4a1-48626f87de0c" />

Sometimes you need to press the BOOT button to flash the firmware

### Connection to app

Open Settings, then "Connect external GPS receiver..."

Scan and connect "AGRO-GPS" device.
