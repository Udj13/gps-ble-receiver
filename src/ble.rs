use esp32_nimble::utilities::mutex::Mutex;
use esp32_nimble::{uuid128, BLECharacteristic, BLEDevice, BLEReturnCode, NimbleProperties};
use esp_idf_hal::delay::FreeRtos;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};

pub struct Ble {
    tx: Sender<String>,
}

impl Ble {
    pub fn new(name: &str) -> Ble {
        let ble_device = BLEDevice::take();
        let ble_advertising = ble_device.get_advertising();
        let server = ble_device.get_server();

        println!("{}, start BLE configuration", name);

        server.on_connect(|server, desc| {
            println!("BLE client connected");

            server
                .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
                .expect("Can't update BLE connection parameters");
        });

        server.on_disconnect(|_desc, reason| {
            println!("BLE client disconnected ({:?})", BLEReturnCode(reason as _));
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
            .set_value(b"GNSS BLE receiver.");

        ble_advertising.stop().expect("Can't stop BLE advertising");

        ble_advertising.name(name).add_service_uuid(service_uuid);

        FreeRtos::delay_ms(10); // time to set settings

        ble_advertising // service uuid
            .start()
            .expect("Can't start BLE advertising");

        println!("Starting BLE thread");
        let (tx, rx) = mpsc::channel::<String>();
        Self::start_ble_thread(notifying_characteristic, rx);

        println!("BLE ready");
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
                    println!("Chunk {}: {}", i + 1, chunk);

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
