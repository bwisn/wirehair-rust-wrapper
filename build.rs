extern crate cc;

fn main() {
    let arch = build_target::target_arch().unwrap();
    let mut binding = cc::Build::new();
    let build = binding
        .cpp(true)
        .file("src/wirehair/wirehair.cpp")
        .file("src/wirehair/gf256.cpp")
        .file("src/wirehair/WirehairCodec.cpp")
        .file("src/wirehair/WirehairTools.cpp")
        .include("src/wirehair")
        .flag("-march=native")
        .flag("-mtune=native")
        .shared_flag(true);

    match arch {
        build_target::Arch::X86_64 | build_target::Arch::X86 => {
            build.flag("-msse4.1");
        }
        _ => {}
    }
    build.shared_flag(true);
    build.compile("wirehair");
}
