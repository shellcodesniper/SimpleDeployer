FROM rust:alpine

RUN echo "RUST VERSION --- $(rustc -V)"

RUN mkdir /app
WORKDIR /app
ADD . /app

RUN cargo build --release --verbose --target-dir "/app/dist"


ENTRYPOINT [ "/app/dist/release/simple-deployer" ]