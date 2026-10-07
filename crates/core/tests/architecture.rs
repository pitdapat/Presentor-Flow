//! Enforces PLAN §3.2: `presenter-core` must never depend on UI or OS crates.
//! Cargo already makes such imports fail to compile; this test catches the
//! dependency being *added* to the manifest in the first place.

const FORBIDDEN: &[&str] = &["egui", "eframe", "winit", "windows", "windows-sys"];

#[test]
fn core_has_no_ui_or_os_dependencies() {
    let manifest = include_str!("../Cargo.toml");
    let mut in_deps = false;
    for line in manifest.lines().map(str::trim) {
        if line.starts_with('[') {
            in_deps = line.contains("dependencies");
            continue;
        }
        if !in_deps || line.starts_with('#') {
            continue;
        }
        let name = line.split(['=', '.', ' ']).next().unwrap_or_default();
        assert!(
            !FORBIDDEN.contains(&name),
            "presenter-core must not depend on `{name}` (PLAN §3.2)"
        );
    }
}
