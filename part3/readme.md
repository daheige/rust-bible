# part3
    redis-demo redis基本使用
    redis-async redis tokio async使用
    redis-pool redis连接池模式
    pulsar-publish pulsar 发送者使用
    pulsar-consumer pulsar 消费者使用
    pulsar-batch pulsar spawn publish sub
    mysql-demo mysql库使用
    sqlx-demo sqlx库使用

# pulsar in docker

    run pulsar in docker
```shell
    docker run -dit \
    --name pulsar-sever \
    -p 6650:6650 \
    -p 8080:8080 \
    --mount source=pulsardata,target=/pulsar/data \
    --mount source=pulsarconf,target=/pulsar/conf \
    apachepulsar/pulsar:2.7.4 \
    bin/pulsar standalone
```
