FROM rust

WORKDIR /usr/src/bob

COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY workspace workspace

COPY --from=docker /usr/local/bin/docker /usr/local/bin/

RUN cargo install --path .

CMD ["bob"]
