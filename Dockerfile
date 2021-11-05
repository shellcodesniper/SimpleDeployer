FROM rust:alpine
RUN apk add --no-cache build-base tzdata openssl openssl-dev
RUN echo "RUST VERSION --- $(rustc -V)"

RUN mkdir /app
WORKDIR /app
ADD . /app

RUN cargo build --release --verbose --target-dir "/app/dist" && mv /app/dist/release/simple-deployer /app/entrypoint && rm -rf /app/dist/

ENTRYPOINT [ "/app/entrypoint" ]
