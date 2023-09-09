# Kafka Consumer/Producer Example

This is a simple example of having a consumer and producer service communicate
via Kafka.

## Get Kafka up

This project uses `docker-compose` to start Kafka and Zookeeper in containers:

```sh
docker-compose -d
```

For my own convenience, I noted the container Id for the Kafka node and created
an alias to execute commands inside the container:

```sh
sudo docker ps
KAFKA_CONTAINER_ID=COPIED_FROM_OUTPUT_ABOVE

alias kafka_exec="sudo docker exec -it $KAFKA_CONTAINER_ID"
```

Finally, I create a Kafka topic:

```sh
kafka_exec /opt/bitnami/kafka/bin/kafka-topics.sh --create --topic topic-name --bootstrap-server localhost:9092
```

## Get Services Up

You can run the individual services by visiting the respective directories and
executing `cargo run`.

Depending on whether you've configured the consumer to read from the beginning
of history or the latest event, the consumer may either pick up every event or
just the events seen after its startup.
