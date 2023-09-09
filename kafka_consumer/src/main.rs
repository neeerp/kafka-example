use kafka::consumer::{Consumer, FetchOffset};
use std::str;

const KAFKA_HOST: &str = "localhost:29092";

fn main() {
    let hosts = vec![KAFKA_HOST.to_owned()];

    let mut consumer = Consumer::from_hosts(hosts)
        .with_topic("topic-name".to_owned())
        .with_fallback_offset(FetchOffset::Latest)
        .create()
        .unwrap();

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}", str::from_utf8(m.value).unwrap());
            }

            consumer.consume_messageset(ms).unwrap();
        }
        consumer.commit_consumed().unwrap();
    }
}
