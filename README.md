# protos

Protobuf specs for all the repos

## Usage

You can either compile when needed using CMake(for C++) or Prost(for Rust)
or use `generated/`

### generated: Rust

- install protoc(eg use prebuilt binaries, cf CI); CHECK: `$PROTOC --version`
- cargo install protoc-gen-prost

- `cd protos/`
- `$PROTOC -I . --prost_out=generated/rust api_circuits/api.proto api_circuits/circuits_routes.proto`
- `$PROTOC -I . --prost_out=generated/rust api_garble/api.proto api_garble/garble_routes.proto`
- `$PROTOC -I . --prost_out=generated/rust skcd/skcd.proto`