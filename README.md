# Master-Redis (masteredis)

An auto-switch proxy for redis master node.

## Core middleware

- socat

```bash
socat TCP4-LISTEN:6379,reuseaddr,fork TCP4:127.0.0.1:6379
```

## Getting Started

Start with command (*Example*):

```bash
docker run -d \
  --restart=always \
  --name=masteredis \
  -p '6399:6379' \
  -e 'REDIS_HOST=127.0.0.1' \
  -e 'REDIS_PORT=6379' \
  -e 'REDIS_USER=user' \
  -e 'REDIS_PASSWORD=password' \
  -e 'CHECK_INTERVAL=5000' \
  repigeons/masteredis
```

### Environment variables

|variable      |type   |required|default|description|example|
|:------------:|:-----:|:------:|:-----:|:---------:|:-----:|
|REDIS_HOST    |string |true    | -     |any one redis node host.|127.0.0.1|
|REDIS_PORT    |integer|true    | -     |any one redis node port.|6379|
|REDIS_USER    |string |false   | -     |redis username, if set.||
|REDIS_PASSWORD|string |false   | -     |redis password, if set.||
|CHECK_INTERVAL|integer|false   |5000   |the interval to check master. (milliseconds) |5000|
