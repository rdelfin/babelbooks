FROM jdrouet/rust-nightly:buster-slim

WORKDIR /usr/src/babelbooks
COPY . .

RUN cargo install --path .

CMD ["api"]
