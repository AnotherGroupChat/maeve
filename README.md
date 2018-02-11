# Maeve [![Build Status](https://travis-ci.org/AnotherGroupChat/maeve.svg?branch=master)](https://travis-ci.org/AnotherGroupChat/maeve)

Maeve is an interpreter for text-based games. The end goal is support for a community-made game based on the Westworld TV series.

### How to build the game:

Go to [the Rust website] and download Rust.

In theory, the rust protobuf compiler should be autodownloaded for you and provisioned.
The current game relies of Protobuf3; exisiting compilers may not support this.
For reference, please see the [PROST!](https://github.com/danburkert/prost) page for details.

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
