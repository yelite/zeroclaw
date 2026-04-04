use std::fs;
use std::path::Path;

fn main() {
    let dist_dir = Path::new("web/dist");

    // Tell Cargo to re-run this script when bundled assets change.
    println!("cargo:rerun-if-changed=web/dist");
    println!("cargo:rerun-if-changed=docs/assets/zeroclaw-trans.png");

    // In Nix builds (and other pure/sandboxed environments) the web frontend
    // must be pre-built before `cargo build` runs.  We no longer invoke npm
    // from build.rs because that requires network access and impure tooling
    // which breaks reproducible builds.
    ensure_dist_dir(dist_dir);
    ensure_dashboard_assets(dist_dir);
}

/// Ensure the dist directory exists so `rust-embed` does not fail at compile
/// time even when the web frontend is not built.
fn ensure_dist_dir(dist_dir: &Path) {
    if !dist_dir.exists() {
        fs::create_dir_all(dist_dir).expect("failed to create web/dist/");
    }
}

fn ensure_dashboard_assets(dist_dir: &Path) {
    // The Rust gateway serves `web/dist/` via rust-embed under `/_app/*`.
    // Ensure the logo asset is present in `web/dist/` at compile time.
    let src = Path::new("docs/assets/zeroclaw-trans.png");
    if !src.exists() {
        eprintln!(
            "cargo:warning=docs/assets/zeroclaw-trans.png not found; skipping dashboard asset copy"
        );
        return;
    }

    let dst = dist_dir.join("zeroclaw-trans.png");
    if let Err(e) = fs::copy(src, &dst) {
        eprintln!("cargo:warning=Failed to copy zeroclaw-trans.png into web/dist/: {e}");
    }
}
