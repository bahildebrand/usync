FROM rust:latest as builder
RUN rustup default nightly
RUN rustup target add thumbv7em-none-eabihf

COPY . /usync
WORKDIR /usync
RUN cargo build --examples

FROM antmicro/renode

COPY --from=builder /usync /usync
WORKDIR /usync

RUN chmod +x entry.sh

ENTRYPOINT ["bash", "./entry.sh"]