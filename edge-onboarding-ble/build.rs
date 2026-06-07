fn main() {
    let mut gen = micropb_gen::Generator::new();
    gen.configure(
        ".",
        micropb_gen::Config::new()
            .string_type("::std::string::String")
            .max_len(128)
            .enum_int_size(micropb_gen::config::IntSize::S32),
    );
    gen.compile_protos(&["onboarding.proto"], "src/proto.rs")
        .unwrap();
}
