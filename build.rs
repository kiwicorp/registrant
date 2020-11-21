fn main() {
    tonic_build::compile_protos("api/registry/registry.proto").unwrap();
    tonic_build::compile_protos("api/sandhog/sandhog.proto").unwrap();
}