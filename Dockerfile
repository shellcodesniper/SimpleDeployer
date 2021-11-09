FROM rust:alpine as build
RUN apk add --no-cache musl-dev tzdata perl gcc g++ make libffi-dev libtool autoconf automake curl

RUN mkdir /app
WORKDIR /app

RUN echo "RUST VERSION --- $(rustc -V)"

RUN cargo init ./ --bin --edition 2018

COPY ./Cargo.lock /app/Cargo.lock
COPY ./Cargo.toml /app/Cargo.toml

RUN cargo build --release --target x86_64-unknown-linux-musl --verbose --target-dir "/app/dist" --features vendored
RUN rm src/*.rs

COPY ./src /app/src

RUN rm /app/dist/release/deps/simple_deployer*
RUN cargo build --release --target x86_64-unknown-linux-musl --verbose --target-dir "/app/dist" --features vendored && mv /app/dist/x86_64-unknown-linux-musl/release/simple-deployer /app/entrypoint && rm -rf /app/dist/

# our final base
FROM rust:alpine

# copy the build artifact from the build stage
COPY --from=build /app/entrypoint .

ENTRYPOINT [ "/app/entrypoint" ]
