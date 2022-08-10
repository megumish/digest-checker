FROM rust:1.62.1

WORKDIR /usr/src/digest-checker
COPY . .

RUN cargo install --path .

CMD ["digest-checker"]