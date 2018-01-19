# Maeve

Maeve is a text-based game based on the Westworld TV series.

### How to build the game:

Go to [the Rust website] and download Rust.

Then once you've downloaded Rust:

```sh
$ git clone https://github.com/AnotherGroupChat/maeve
$ cd maeve
$ cargo install protobuf
```

- Ubuntu
```sh
$ sudo apt-get install protobuf-compiler
```

- Arch
```sh
$ sudo pacman -S protobuf
```

- Mac
```sh
$ brew install protobuf
```

```sh
$ ./build.sh
$ cargo build
```

[the Rust website]: <https://www.rust-lang.org/en-US/>
