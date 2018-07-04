FROM rust:1.27.0
WORKDIR /usr/src/maeve
COPY . .
RUN rustup install nightly
RUN rustup default nightly
RUN cargo install
CMD ["maeve"]
