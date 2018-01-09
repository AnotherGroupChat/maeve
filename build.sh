#Clear out old generated files
rm src/protos/master.rs

# Builds the game from a text file into the needed binary format.
cat games/sample.pbtxt | protoc --encode=Maeve.Game protos/*.proto > games/sample.pb

# Generate new files
protoc --rust_out src/protos protos/*.proto
