FROM rust:1.31.0
WORKDIR /var/games/maeve
RUN rustup install nightly
RUN rustup default nightly
RUN rustup component add rustfmt --toolchain nightly
RUN cargo install sccache

RUN usermod -u 1000 games
RUN chown -R games:games /var/games
CMD ["/bin/bash"]
