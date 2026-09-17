#[cfg(not(any(
    target_arch = "wasm32",
    all(target_os = "macos", feature = "macos-native"),
    all(target_os = "windows", feature = "windows-native"),
)))]
mod ui;

#[cfg(not(any(
    target_arch = "wasm32",
    all(target_os = "macos", feature = "macos-native"),
    all(target_os = "windows", feature = "windows-native"),
)))]
mod platform;

#[cfg(not(any(
    target_arch = "wasm32",
    all(target_os = "macos", feature = "macos-native"),
    all(target_os = "windows", feature = "windows-native"),
)))]
mod runtime;

#[cfg(not(any(
    target_arch = "wasm32",
    all(target_os = "macos", feature = "macos-native"),
    all(target_os = "windows", feature = "windows-native"),
)))]
fn main() {
    runtime::run();
}

#[cfg(all(target_os = "macos", feature = "macos-native"))]
fn main() {
    eprintln!(
        "This binary is for Terminal. Use the macOS-native binary for the macOS version, or build without the 'macos-native' feature."
    );
}

#[cfg(all(target_os = "windows", feature = "windows-native"))]
fn main() {
    eprintln!(
        "This binary is for Terminal. Use the Windows-native binary for the Windows version, or build without the 'windows-native' feature."
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {}
