# protos

Protobuf specs for all the repos

## Usage

You can either compile when needed using CMake(for C++) or Prost(for Rust)
or use `generated/`

### generated: Rust

- install protoc(eg use prebuilt binaries, cf CI); CHECK: `protoc --version`
- `cargo install protoc-gen-prost`

- `cd protos/`
- `protoc -I . --prost_out=generated/rust api_circuits/api.proto api_circuits/circuits_routes.proto`
- `protoc -I . --prost_out=generated/rust --prost_opt=retain_enum_prefix=false --prost_opt=btree_map="." garbled/garbled.proto`
- `protoc -I . --prost_out=generated/rust --prost_opt=retain_enum_prefix=false skcd/skcd.proto`