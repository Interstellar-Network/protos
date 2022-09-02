# protos

Protobuf specs for all the repos

## Usage

You can either compile when needed using CMake(for C++) or Prost(for Rust)
or use `generated/`

### generated: Rust

- cd protos
- install protoc(eg use prebuilt binaries, cf CI)
- cargo install protoc-gen-prost
- protoc -I . --prost_out=generated/rust api_circuits/api.proto api_circuits/circuits_routes.proto
- protoc -I . --prost_out=generated/rust api_garble/api.proto api_garble/garble_routes.proto