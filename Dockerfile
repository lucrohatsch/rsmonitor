FROM rust:1.67

WORKDIR /usr/src/rsmonitor
COPY . .


RUN cargo build --release

CMD cargo run
