// Copyright (c) 2023 MASSA LABS <info@massa.net>
//
//! ## **Overview**
//!
//! This module contains the generated source code from protobuf files for Massa blockchain.
//! It uses utilizes the `tonic-build` tool to generate Rust code from the Protobuf definitions.
//!
//! ## **Structure**
//!
//! * `build.rs`: This file contains build instructions for generating Rust code from the Protobuf definitions using the `tonic-build` tool.
//! * `massa-proto`: This git submodule contains the Protobuf message definitions for the Massa blockchain API
//! * `src/`: This directory contains the generated Rust code for the Protobuf message definitions.
//! It also includes a `_XXX_includes.rs` files for importing the generated Rust modules and an `api.bin` file for gRPC server reflection protocol.
//!
//! ## **Usage**
//! To use this module, simply include it as a dependency in your Rust project's `Cargo.toml` file.
//! You can then import the necessary Rust modules for the Massa API and use the Protobuf messages as needed.
//!
#![warn(unused_crate_dependencies)]
#![cfg_attr(not(feature = "tonic"), no_std)]
use prost_types as _;

#[cfg(feature = "tonic")]
/// Google protos Module
pub mod google {
    /// Google API Module
    pub mod api {
        include!("google.api.rs");
    }
    /// Google RPC Module
    pub mod rpc {
        include!("google.rpc.rs");
    }
}

/// Massa protos Module
pub mod massa {
    /// Massa ABI Module
    pub mod abi {
        /// Version 1 of the Massa ABI protos
        pub mod v1 {
            include!("massa.abi.v1.rs");
        }
    }
    #[cfg(feature = "tonic")]
    /// Massa API Module
    pub mod api {
        /// Version 1 of the Massa API protos
        pub mod v1 {
            include!("massa.api.v1.rs");
            /// Compiled file descriptor set for the Massa public service protos
            pub const FILE_DESCRIPTOR_SET_PUBLIC: &[u8] = include_bytes!("api_public.bin");
            /// Compiled file descriptor set for the Massa private service protos
            pub const FILE_DESCRIPTOR_SET_PRIVATE: &[u8] = include_bytes!("api_private.bin");
        }
    }

    /// Massa Model Module
    pub mod model {
        /// Version 1 of the Massa Model protos
        pub mod v1 {
            include!("massa.model.v1.rs");
        }
    }
}
