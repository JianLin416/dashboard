use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::env;
use std::time::Duration;

pub async fn connect_mqtt(id: String, host: String, port: u16) -> Option<AsyncClient> {
    let mut mqttoptions = MqttOptions::new(id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    match eventloop.poll().await {
        Ok(Event::Incoming(Packet::ConnAck(_))) => {
            tokio::spawn(async move {
                loop {
                    if let Err(_) = eventloop.poll().await {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            });
            Some(client)
        }
        _ => None,
    }
}

pub async fn publish_message(client: &AsyncClient, message: &str) {
    let topic: String = env::var("MQTT_TOPIC").unwrap().parse().unwrap();
    eprintln!("topic: {}", topic);

    let _ = client
        .publish(topic, QoS::AtLeastOnce, false, message)
        .await;
}
