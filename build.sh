#!/bin/bash
# TODO(#21): Replace file with build.rs

# either use 'game_design' as the default or
# a user supplied name
filename=${1:-"game_design"}

#Clear out old generated files
if [ -z "src/protos/master.rs" ]; then
  rm src/protos/master.rs
fi

# Builds the game from a text file into the needed binary format.
cat games/$filename.pbtxt | protoc --encode=Maeve.Game protos/*.proto > games/$filename.pb

# Generate new files
protoc --rust_out src/protos protos/*.proto
