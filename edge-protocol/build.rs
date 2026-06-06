fn main() {
    let mut gen = micropb_gen::Generator::new();
    gen.use_container_heapless();
    gen.configure(".", 
        micropb_gen::Config::new()
            .max_bytes(6)
            .max_len(16)
            .enum_int_size(micropb_gen::config::IntSize::S8)
    );
    gen.compile_protos(&["v2.proto"], "src/v2_proto.rs").unwrap();
}