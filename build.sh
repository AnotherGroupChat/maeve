#Clear out old generated files
if [ -z "src/protos/master.rs" ]; then
  rm src/protos/master.rs
fi

# Builds the game from a text file into the needed binary format.
cat games/game_design.pbtxt | protoc --encode=Maeve.Game protos/*.proto > games/game_design.pb

# Generate new files
protoc --rust_out src/protos protos/*.proto
