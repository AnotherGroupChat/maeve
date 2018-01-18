#Clear out old generated files
rm src/protos/master.rs

# Builds the game from a text file into the needed binary format.
cat games/game_design.pbtxt | protoc --encode=Maeve.Game protos/*.proto > games/game_design.pb

# Generate new files
protoc --rust_out src/protos protos/*.proto
