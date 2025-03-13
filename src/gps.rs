use colored::*;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::prelude::Hertz;
use esp_idf_hal::uart::{UartDriver, UART1};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub struct Gps {
    pub rx: Receiver<String>,
}

impl Gps {
    pub fn new(uart1: UART1, tx: AnyIOPin, rx: AnyIOPin) -> Gps {
        println!("Starting UART");
        let config = esp_idf_hal::uart::config::Config::new().baudrate(Hertz(115_200));
        let uart = UartDriver::new(
            uart1,
            tx,
            rx,
            Option::<gpio::Gpio0>::None,
            Option::<gpio::Gpio1>::None,
            &config,
        )
        .expect("GPS - uart driver error");

        // Disable a NMEA messages
        let init_commands = [
            [
                0xF1, 0xD9, 0x06, 0x01, 0x03, 0x00, 0xF0, 0x01, 0x00, 0xFB, 0x11,
            ],
            [
                0xF1, 0xD9, 0x06, 0x01, 0x03, 0x00, 0xF0, 0x02, 0x00, 0xFC, 0x13,
            ],
            [
                0xF1, 0xD9, 0x06, 0x01, 0x03, 0x00, 0xF0, 0x04, 0x00, 0xFE, 0x17,
            ],
            [
                0xF1, 0xD9, 0x06, 0x01, 0x03, 0x00, 0xF0, 0x06, 0x00, 0x00, 0x1B,
            ],
            [
                0xF1, 0xD9, 0x06, 0x01, 0x03, 0x00, 0xF0, 0x07, 0x00, 0x01, 0x1D,
            ],
            [
                0xF1, 0xD9, 0x06, 0x01, 0x03, 0x00, 0xF0, 0x08, 0x00, 0x02, 0x1F,
            ],
            [
                0xF1, 0xD9, 0x06, 0x01, 0x03, 0x00, 0xF0, 0x20, 0x00, 0x1A, 0x4F,
            ],
        ];

        for (i, command) in init_commands.iter().enumerate() {
            match uart.write(command) {
                Ok(_) => println!("GPS initialization command {} sent successfully", i + 1),
                Err(e) => println!(
                    "Failed to send GPS initialization command {}: {:?}",
                    i + 1,
                    e
                ),
            }
            thread::sleep(Duration::from_millis(100));
        }

        // Set NMEA output frequency 10Hz
        let init_command = [
            0xF1, 0xD9, 0x06, 0x42, 0x14, 0x00, 0x00, 0x0A, 0x38, 0x00, 0x64, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x24,
        ];

        match uart.write(&init_command) {
            Ok(_) => println!("GPS initialization command sent successfully"),
            Err(e) => println!("Failed to send GPS initialization command: {:?}", e),
        }
        thread::sleep(Duration::from_millis(100));

        let mut nmea_buffer = String::with_capacity(100);
        let mut buf = [0_u8; 1];

        let (tx, rx) = mpsc::channel::<String>();

        std::thread::spawn(move || loop {
            uart.read(&mut buf, BLOCK).expect("UART GPS can't read");
            nmea_buffer.push(char::from(buf[0]));

            if nmea_buffer.ends_with("\r\n") {
                if nmea_buffer.contains("GNGGA") || nmea_buffer.contains("GNRMC") {
                    println!("{}", nmea_buffer.yellow());
                    tx.send(nmea_buffer.clone())
                        .expect("Can't send from GPS thread");
                    nmea_buffer.clear();
                } else {
                    //print!("{}", nmea_buffer.green());
                }
                nmea_buffer.clear();
            }
        });

        Gps { rx }
    }
}
