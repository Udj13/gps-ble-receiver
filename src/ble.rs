use esp32_nimble::utilities::mutex::Mutex;
use esp32_nimble::{uuid128, BLECharacteristic, BLEDevice, BLEReturnCode, NimbleProperties};
use std::sync::Arc;

pub struct Ble {
    notifying_characteristic: Arc<Mutex<BLECharacteristic>>,
}

impl Ble {
    pub fn new() -> Ble {
        let ble_device = BLEDevice::take();
        let ble_advertising = ble_device.get_advertising();
        let server = ble_device.get_server();

        server.on_connect(|server, desc| {
            println!("Client connected");

            server
                .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
                .expect("Cant't update connection parameters");
        });

        server.on_disconnect(|_desc, reason| {
            println!("Client disconnected ({:?})", BLEReturnCode(reason as _));
        });

        let service_uuid = uuid128!("aea7d00f-ae87-47b4-bc55-cada2dbdf1f4"); //???
        let service = server.create_service(service_uuid);

        let notifying_characteristic = service.lock().create_characteristic(
            uuid128!("bd914a74-c7c9-486d-9142-fd5fba002bcc"),
            NimbleProperties::READ | NimbleProperties::NOTIFY,
        );
        notifying_characteristic.lock().set_value(b"Initial value.");

        ble_advertising
            .name("AGRO NAVIGATION")
            .add_service_uuid(uuid128!("aea7d00f-ae87-47b4-bc55-cada2dbdf1f4"))
            .start()
            .unwrap();

        Ble {
            notifying_characteristic,
        }
    }

    pub fn send(&mut self, output: &str) {
        println!("Send to BLE:");

        let text_chunks = Self::split_text_into_chunks(output);

        for (i, chunk) in text_chunks.iter().enumerate() {
            println!("Chunk {}: {}", i + 1, chunk);

            self.notifying_characteristic
                .lock()
                .set_value(chunk.as_bytes());
        }
    }

    fn split_text_into_chunks(text: &str) -> Vec<&str> {
        let mut chunks = Vec::new();
        let mut remaining_text = text;

        while !remaining_text.is_empty() {
            let chunk = &remaining_text[..23.min(remaining_text.len())];
            chunks.push(chunk);
            remaining_text = &remaining_text[chunk.len()..];
        }

        chunks
    }
}
