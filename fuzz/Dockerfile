# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /utils/

WORKDIR /utils/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>

COPY --from=builder /utils/fuzz/target/x86_64-unknown-linux-gnu/release/pad_iso7816 /

COPY --from=builder /utils/fuzz/target/x86_64-unknown-linux-gnu/release/pad_iso10126 /

COPY --from=builder /utils/fuzz/target/x86_64-unknown-linux-gnu/release/pad_ansix923 /

COPY --from=builder /utils/fuzz/target/x86_64-unknown-linux-gnu/release/pad_pkcs7 /

COPY --from=builder /utils/fuzz/target/x86_64-unknown-linux-gnu/release/pad_zeros /

COPY --from=builder /utils/fuzz/target/x86_64-unknown-linux-gnu/release/pad_no_padding /