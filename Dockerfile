FROM rust:1.55

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["lithium"]

