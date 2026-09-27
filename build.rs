use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=bridge/apple_vision_bridge.swift");

    #[cfg(target_os = "macos")]
    {
        let bridge_src = "bridge/apple_vision_bridge.swift";
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let bridge_bin = format!("{}/apple_vision_bridge", out_dir);

        if Path::new(bridge_src).exists() {
            let status = Command::new("swiftc")
                .args([
                    "-parse-as-library",
                    "-O",
                    bridge_src,
                    "-o",
                    &bridge_bin,
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("cargo:rustc-env=APPLE_VISION_BRIDGE_BIN={}", bridge_bin);
                }
                _ => {
                    println!("cargo:warning=Failed to compile Swift vision bridge with swiftc");
                }
            }
        }
    }
}
