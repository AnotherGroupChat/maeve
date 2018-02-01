# Maeve [![Build Status](https://travis-ci.org/AnotherGroupChat/maeve.svg?branch=master)](https://travis-ci.org/AnotherGroupChat/maeve)

Maeve is an interpreter for text-based games. The end goal is support for a community-made game based on the Westworld TV series.

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

then add `protoc-gen-rust` to $PATH (see [rust-protobuf](https://github.com/stepancheg/rust-protobuf) for more details)
```
$ PATH="$HOME/.cargo/bin:$PATH"
```

and then compile with
```
$ ./build.sh
$ cargo build
```

Once you've completed the above instructions the game should now be ready to run with:

```sh
$ cargo run
```

When starting a new game, you will be asked which file you would like to load, currently the file you should
choose is:
```sh
games/game_design.pb
```
After you have selected your file, the game will ask you where you would like to create a new save file; you
can save it wherever you like.

#### Compilation Flags:
| Flag                   | Description    |
| ------------------     | -------------: |
| --features=pretty      | Implements readlib, which provides a user-friendly cli. |
| --features=default     | Implements stdio and stdout, which provides a not-so-user-friendly cli. |

#### Runtime Flags:
| Flag                   | Description    |
| ------------------     | -------------: |
| --new=\<game file\>      | Provides a game file which will start a new game. |
| --load=\<load file\>     | Provides a load file which will continue a game. |

### Talking to the Interpreter:

to be continued...

### Creating your own games:

to be continued...


[the Rust website]: <https://www.rust-lang.org/en-US/>
