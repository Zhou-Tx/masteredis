FROM rust:alpine AS Builder

WORKDIR /opt
COPY / /opt
RUN cargo build --release

FROM alpine AS Runner

RUN apk add socat --no-cache && \
    cp /usr/bin/socat /usr/bin/masteredis-socat

COPY --from=Builder /opt/target/release/masteredis /usr/bin/

CMD ["masteredis"]
