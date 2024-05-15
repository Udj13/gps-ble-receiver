use esp32_nimble::utilities::mutex::Mutex;
use esp32_nimble::{uuid128, BLEAdvertisementData, BLECharacteristic, BLEDevice, NimbleProperties};
use esp_idf_hal::delay::FreeRtos;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use colored::*;
use esp32_nimble::enums::{AuthReq, SecurityIOCap};

pub struct Ble {
    tx: Sender<String>,
}


impl Ble {
    pub fn new(name: &str) -> Ble {
        let ble_device = BLEDevice::take();
        ble_device  // iOs settings
            .security()
            .set_auth(AuthReq::Bond) // Bonding enables key storage for reconnection
            .set_io_cap(SecurityIOCap::NoInputNoOutput) // You can choose any IO capability
            .resolve_rpa(); // Crucial for managing iOS's dynamic Bluetooth addresses

        let ble_advertising = ble_device.get_advertising();
        let server = ble_device.get_server();

        println!("{}, start BLE configuration", name);

        server.on_connect(|server, desc| {
            println!("{}", "BLE client connected".on_blue());

            server
                .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
                .expect("Can't update BLE connection parameters");
        });

        server.on_disconnect(|_desc, _reason| {
            println!("{}", "BLE client disconnected".on_blue());
        });

        let service_uuid = uuid128!("0000ffa2-ae87-47b4-bc55-cada2dbdf1f4");
        let service = server.create_service(service_uuid);

        let characteristic_uuid = uuid128!("0000ffe1-c7c9-486d-9142-fd5fba002bcc");
        let notifying_characteristic = service.lock().create_characteristic(
            characteristic_uuid,
            NimbleProperties::READ | NimbleProperties::NOTIFY,
        );
        notifying_characteristic
            .lock()
            .set_value(b"AGRO-GPS");

        let mut ad_data = BLEAdvertisementData::new();
        ad_data.
            name(name). // main.rs
            add_service_uuid(service_uuid);

        ble_advertising
            .lock()
            .set_data(&mut ad_data)
            .expect("Can't set BLE advertising");

        FreeRtos::delay_ms(10); // time to set settings

        ble_advertising
            .lock() // service uuid
            .start()
            .expect("Can't start BLE advertising");

        println!("{}", "Local GATTS info:".on_blue());
        server.ble_gatts_show_local();

        println!("{}", "Starting BLE thread".on_blue());
        let (tx, rx) = mpsc::channel::<String>();
        Self::start_ble_thread(notifying_characteristic, rx);

        println!("{}", "BLE ready".on_blue());
        println!("Characteristic: {}", characteristic_uuid);
        println!("Service: {}", service_uuid);
        println!();

        Ble { tx }
    }

    fn start_ble_thread(
        notifying_characteristic: Arc<Mutex<BLECharacteristic>>,
        rx: Receiver<String>,
    ) {
        const CHUNK_SIZE: usize = 21;

        std::thread::spawn(move || loop {
            if let Ok(output) = rx.recv() {
                let output: &str = output.as_str();
                print!("BLE thread, received {}", output);

                let text_chunks = Self::split_text_into_chunks(output, CHUNK_SIZE);

                for (i, chunk) in text_chunks.iter().enumerate() {
                    println!("Chunk {}: {}", i + 1, chunk.on_blue());

                    notifying_characteristic
                        .lock()
                        .set_value(chunk.as_bytes())
                        .notify();

                    FreeRtos::delay_ms(10);
                }
            }
            println!("BLE thread idle\n");
        });
    }

    pub fn send(&mut self, output: &str) {
        self.tx
            .send(output.parse().unwrap())
            .expect("Error while sending to BLE thread");
    }

    fn split_text_into_chunks(text: &str, chunk_size: usize) -> Vec<&str> {
        let mut chunks = Vec::new();
        let mut remaining_text = text;

        while !remaining_text.is_empty() {
            let chunk = &remaining_text[..chunk_size.min(remaining_text.len())];
            chunks.push(chunk);
            remaining_text = &remaining_text[chunk.len()..];
        }

        chunks
    }
}
