fn main() {
    // tell rustc to link to SDL2 and SDL2_ttf
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_ttf");

    // tell rustc where to look for them
    println!("cargo:rustc-link-search=native=libs");
}
