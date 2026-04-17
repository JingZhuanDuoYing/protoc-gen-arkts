# build
cargo build

# Generation of ArkTS file.
# Add current target/debug to the front of PATH so protoc finds the local build first
export PATH=$(pwd)/target/debug:$PATH

protoc -I /usr/local/include -I ./tests --arkts_out=./tests ./tests/gen.proto --arkts_opt=with_sendable=true
protoc -I /usr/local/include -I ./tests --arkts_out=./tests ./tests/common.proto --arkts_opt=with_sendable=true
protoc -I /usr/local/include -I ./tests --arkts_out=./tests ./tests/enum.proto --arkts_opt=with_sendable=true

# Test generated ArkTS file.
# Wait till we get an ArkTs runtime from Huawei.
