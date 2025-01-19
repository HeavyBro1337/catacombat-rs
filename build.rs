use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=build.rs");

    let assets_dir = Path::new("assets");

    // Определяем целевую платформу
    let target = env::var("CARGO_BUILD_TARGET").unwrap_or_else(|_| env::var("TARGET").unwrap());

    // Определяем директорию, где будет находиться бинарник
    let target_dir = "target";
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_assets_dir = Path::new(target_dir).join(target).join(profile).join("assets");

    println!("Target dir: {}", target_assets_dir.display());

    if let Err(e) = fs::create_dir_all(&target_assets_dir) {
        panic!("Failed to create the assets folder: {}", e);
    }

    if let Err(e) = copy_recursively(assets_dir, &target_assets_dir) {
        panic!("Failed to copy files: {}", e);
    }

}

fn copy_recursively(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = dst.join(entry.file_name());

            copy_recursively(&entry_path, &dest_path)?;
        }
    } else {
        fs::copy(src, dst)?;
    }
    Ok(())
}
