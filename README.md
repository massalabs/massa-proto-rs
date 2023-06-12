# MASSA PROTO RS

Generated Rust source code from protobuf files for Massa blockchain.

Requirements
------------

Make sure you have the following latest versions of the required tools:

- [protoc](https://grpc.io/docs/protoc-installation/): `3.21.12+`.

Please ensure that you have the required versions or newer to guarantee compatibility and access to the latest features.

Development
-----------
In you want to build locally, you first need to get the `massa-proto` submodule that contains `.proto`.

Either clone `massa-proto-rs` this way
```bash
git clone --recurse-submodules https://github.com/massalabs/massa-proto-rs.git
```
Or if you change your mind and have already cloned `massa-proto-rs`, you can get the submodule by running:
```bash
git submodule update --init --force --recursive --remote
```
After the submodule is initialized one can `cd` into the module dir and change branch etc.

Project build
-------------

You can generate source code from protobuf files by running:
```bash
cargo build --features=tonic-build,tonic
```

If there is a change in `_abi_includes.rs` or `_api_includes.rs` files, you need to update `lib.rs`.

Postman integration
-------------------
You can easily import APIs collections from [Massa's Postman workspace](https://www.postman.com/massalabs) and start testing and exploring the provided functionalities by Massa API's.

VSCode settings
------------------

1- Install [vscode-proto3](https://marketplace.visualstudio.com/items?itemName=zxh404.vscode-proto3) extension.

2- The following settings contain a `protoc` configuration block with Java generation output:

```json
{
    "protoc": {  // Specifies the configuration for the protoc plugin.
        "path": "/PATH/TO/PROTOC",  // Sets the path to the protoc binary that will be used to compile the protobuf files.
        "compile_on_save": true,  // Enables automatic compilation of protobuf files when they are saved.
        "options": [  // Specifies the command line options that will be passed to protoc.
            "{workspaceRoot}/massa-proto/proto/**/*.proto",  // Specifies the path to the protobuf files that should be compiled.
            "--proto_path=${workspaceRoot}/massa-proto/proto/",  // Specifies the directory to search for imported protobuf files.
            "--proto_path=${workspaceRoot}/massa-proto/proto/commons",  // Specifies the directory to search for imported common protobuf files.
            "--proto_path=${workspaceRoot}/massa-proto/proto/third_party",  // Specifies the directory to search for imported third_party protobuf files.
            "--java_out=${workspaceRoot}/massa-proto/gen/",  // Generates Java code from the protobuf files.
        ]
    }
}
```

3- Add the snippet above to `.vscode/settings.json`.
