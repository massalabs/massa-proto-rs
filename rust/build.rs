// Copyright (c) 2023 MASSA LABS <info@massa.net>

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic::build()?;

    Ok(())
}

mod tonic {
    use glob::glob;
    use std::{
        env,
        path::{Path, PathBuf},
    };

    /// This function is responsible for building the Massa protobuf API and generating documentation
    pub fn build() -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let parent_dir = current_dir.parent().ok_or("no parent directory found")?;

        let protos_path = parent_dir.join("massa-proto/proto/apis");

        let protos = find_protos(protos_path)?;
        let proto_include_paths = [
            parent_dir.join("massa-proto/proto/apis/massa"),
            parent_dir.join("massa-proto/proto/commons"),
            parent_dir.join("massa-proto/proto/third_party"),
        ];

        tonic_build::configure()
            .build_server(true)
            .build_transport(true)
            .build_client(true)
            .type_attribute(
                ".google.api.HttpRule",
                "#[cfg(not(doctest))]\n\
                 #[allow(dead_code)]\n\
                 pub struct HttpRuleComment{}\n\
                 /// HACK: see docs in [`HttpRuleComment`] ignored in doctest pass",
            )
            .file_descriptor_set_path("src/api.bin")
            .include_file("_includes.rs")
            .out_dir("src/")
            .compile(&protos, &proto_include_paths)
            .map_err(|e| format!("protobuf compilation error: {:?}", e))?;

        Ok(())
    }

    fn find_protos(dir_path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let glob_pattern = format!("{}/**/*.proto", dir_path.as_ref().display());
        let paths: Vec<_> = glob(&glob_pattern)?.filter_map(Result::ok).collect();

        if paths.is_empty() {
            return Err(format!(
                "no .proto files found in the specified directory: {}",
                dir_path.as_ref().display()
            )
            .into());
        }

        Ok(paths)
    }
}
