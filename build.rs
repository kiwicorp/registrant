//! Build script that automatically compiles protos.

const PROTOS: &[&str] = &[
    "selftechio/naas/common.proto",
    "selftechio/naas/naas.proto"
];

fn main() {
    tonic_build::configure()
        .compile(PROTOS, 
        &["api"])
        .unwrap();
}