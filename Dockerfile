FROM rust:alpine3.20

WORKDIR /app

ENV UID=1000
ENV GID=1000
ENV OPENSSL_DIR=/usr

RUN : \
    && apk add --no-cache \
        musl-dev \
        openssl \
        openssl-dev \
        build-base \
        pkgconfig \
        perl \
        make \
        libgcc \
        libstdc++ \
        musl \
    && :

RUN : \
    && cargo install sqlx-cli --no-default-features --features postgres \
    && :

COPY Cargo.toml Cargo.lock ./

RUN : \
    && mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --target x86_64-unknown-linux-musl \
    && rm -rf src \
    && :

ENV SQLX_OFFLINE=true

RUN \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    : \
    && cargo build --target x86_64-unknown-linux-musl \
    && mv /app/target/x86_64-unknown-linux-musl/release/ticky /app/ticky \
    && :

ENV SQLX_OFFLINE=false

CMD ["/app/ticky"]
