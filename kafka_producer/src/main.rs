use kafka::producer::{Producer, Record};

const KAFKA_HOST: &str = "localhost:29092";

fn main() {
    let hosts = vec![KAFKA_HOST.to_owned()];

    let mut producer = Producer::from_hosts(hosts).create().unwrap();

    for i in 0..10 {
        let buf = format!("{i}");
        producer
            .send(&Record::from_value("topic-name", buf.as_bytes()))
            .unwrap();
        println!("Sent: {i}");
    }
}
