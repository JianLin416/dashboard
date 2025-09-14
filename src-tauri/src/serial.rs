use rumqttc::AsyncClient;
use std::time::Duration;
use std::{collections::HashMap, env};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncReadExt;
use tokio_serial::SerialPortBuilderExt;

use crate::mqtt;

pub async fn start_serial_reader(
    app_handle: AppHandle,
    port_name: String,
    mqtt_client: AsyncClient,
) {
    let baud_rate: u32 = env::var("SERIAL_BAUDRATE").unwrap().parse().unwrap();

    let mut port = tokio_serial::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open_native_async()
        .expect("Cannot open serial");

    let mut buffer = [0u8; 1024];
    let mut recv_buffer = String::new();

    tokio::spawn(async move {
        loop {
            match port.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let byte = String::from_utf8_lossy(&buffer[..n]);
                    recv_buffer.push_str(&byte);

                    while let Some(pos) = recv_buffer.find('\n') {
                        let line = recv_buffer.drain(..=pos).collect::<String>();
                        let raw = line.trim();

                        let parsed = parse_key_value(&raw);
                        if !parsed.is_empty() {
                            let _ = app_handle.emit("serial-data", parsed);
                        }

                        let mqtt_client_clone = mqtt_client.clone();

                        mqtt::publish_message(&mqtt_client_clone, raw).await;
                    }
                }
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
        }
    });
}

pub async fn start_serial_reader_without_mqtt(app_handle: AppHandle, port_name: String) {
    let baud_rate: u32 = env::var("SERIAL_BAUDRATE").unwrap().parse().unwrap();

    let mut port = tokio_serial::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open_native_async()
        .expect("Cannot open serial");

    let mut buffer = [0u8; 1024];
    let mut recv_buffer = String::new();

    tokio::spawn(async move {
        loop {
            match port.read(&mut buffer).await {
                Ok(n) if n > 0 => {
                    let byte = String::from_utf8_lossy(&buffer[..n]);
                    recv_buffer.push_str(&byte);

                    while let Some(pos) = recv_buffer.find('\n') {
                        let line = recv_buffer.drain(..=pos).collect::<String>();
                        let raw = line.trim();

                        let parsed = parse_key_value(&raw);
                        if !parsed.is_empty() {
                            let _ = app_handle.emit("serial-data", parsed);
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("error while reading serial, {}", e);
                    break;
                }
            }
        }
    });
}

fn parse_key_value(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for part in input.split(';') {
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    map
}
