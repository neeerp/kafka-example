# Kafka Consumer

This is a simple Kafka Consumer. You can configure it in the code to read either
all messages from the "start of history", or only the latest messages as they
arrive. This can be achieved by modifying the `FetchOffset`, using `Earliest` or
`Latest` accordingly.
