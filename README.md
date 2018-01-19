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
$ ./build.sh
$ cargo build
```

- Arch
```sh
$ sudo pacman -S protobuf
$ ./build.sh
$ cargo build
```

- Mac
```sh
$ brew install protobuf
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
maeve/games/game_design.pb
```
After you have selected your file, the game will ask you where you would like to create a new save file; you
can save it wherever you like.

### Talking to the Interpreter:

to be continued...

### Creating your own games:

to be continued...


[the Rust website]: <https://www.rust-lang.org/en-US/>
