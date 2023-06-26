FROM rust:alpine
WORKDIR /usr/src/todos
COPY Cargo.toml .
COPY ./src ./
VOLUME ["/output", "/usr/local/cargo"]
RUN cargo install --path .
CMD ["todos"]
