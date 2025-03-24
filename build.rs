fn main() {
    slint_build::compile_with_config(
        "src/ui/main.slint",
        slint_build::CompilerConfiguration::new()
            .with_include_paths(vec!["src/ui".into()])
    ).unwrap();
} 