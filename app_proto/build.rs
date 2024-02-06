fn main() {

    #[cfg(feature = "add")]
    tonic_build::configure()
        .out_dir("src")
        .compile(&["proto/add.proto"], &["proto"])
        .expect("Couldn't compile proto files");
    #[cfg(feature = "mul")]
        tonic_build::configure()
            .out_dir("src")
            .compile(&["proto/mul.proto"], &["proto"])
            .expect("Couldn't compile proto files");
    #[cfg(feature = "maths")]
        tonic_build::configure()
            .out_dir("src")
            .compile(&["proto/maths.proto"], &["proto"])
            .expect("Couldn't compile proto files");
}
