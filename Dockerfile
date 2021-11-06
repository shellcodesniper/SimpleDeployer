FROM rust:alpine

RUN apk add --no-cache musl-dev tzdata perl gcc g++ make libffi-dev libtool autoconf automake curl

RUN mkdir /app && mkdir /musl && mkdir /wget
WORKDIR /app
ADD . /app

# RUN cd /wget && wget https://github.com/openssl/openssl/archive/refs/tags/OpenSSL_1_1_1l.tar.gz \
#   && tar zxvf OpenSSL_1_1_1l.tar.gz \
#   && cd openssl-OpenSSL_1_1_1l/ \
#   && CC="musl-gcc -fPIE -pie" ./Configure no-shared no-async --prefix=/musl --openssldir=/musl/ssl linux-x86_64 = \
#   && make depend \ 
#   && make -j$(nproc) \
#   && make install \
#   && cd /app
RUN echo "RUST VERSION --- $(rustc -V)"


# RUN export PKG_CONFIG_ALLOW_CROSS=1 \
#   export OPENSSL_STATIC=true \
#   export OPENSSL_DIR=/musl
RUN cargo build --release --target x86_64-unknown-linux-musl --verbose --target-dir "/app/dist" --features vendored && mv /app/dist/x86_64-unknown-linux-musl/release/simple-deployer /app/entrypoint && rm -rf /app/dist/

ENTRYPOINT [ "/app/entrypoint" ]
