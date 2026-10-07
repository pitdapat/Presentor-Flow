//! Loads the bundled Noto Sans / Noto Sans SC fonts from `assets/fonts`
//! into egui (T0.5).
//!
//! The fonts are read from disk next to the executable instead of being
//! embedded, so the binary stays small (risk R3). During development the
//! repo's `assets/fonts` folder is used.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily};

/// Latin / Cyrillic / Greek font.
pub const LATIN_FILE: &str = "NotoSans-Regular.ttf";
/// Simplified Chinese font, used as fallback for CJK characters.
pub const CHINESE_FILE: &str = "NotoSansSC-Regular.otf";

/// The font family slides are drawn with.
pub fn slide_family() -> FontFamily {
    FontFamily::Name("slide".into())
}

/// Folders searched for the fonts, in order.
fn candidate_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(exe_dir) = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
    {
        dirs.push(exe_dir.join("assets").join("fonts"));
    }
    // `cargo run`: the repo's assets folder, two levels above this crate.
    dirs.push(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("assets")
            .join("fonts"),
    );
    dirs
}

/// Finds the folder that contains both font files.
pub fn find_font_dir() -> Option<PathBuf> {
    candidate_dirs()
        .into_iter()
        .find(|d| d.join(LATIN_FILE).is_file() && d.join(CHINESE_FILE).is_file())
}

/// Builds font definitions with Noto Sans first and Noto Sans SC as the
/// fallback, for both the UI and slides. egui's own fonts stay as the last
/// fallback (for symbols such as ▶).
pub fn definitions(dir: &Path) -> std::io::Result<FontDefinitions> {
    let latin = std::fs::read(dir.join(LATIN_FILE))?;
    let chinese = std::fs::read(dir.join(CHINESE_FILE))?;

    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert("NotoSans".into(), Arc::new(FontData::from_owned(latin)));
    fonts
        .font_data
        .insert("NotoSansSC".into(), Arc::new(FontData::from_owned(chinese)));

    let ours = ["NotoSans".to_owned(), "NotoSansSC".to_owned()];
    let defaults = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    let proportional: Vec<String> = ours.iter().cloned().chain(defaults).collect();
    fonts
        .families
        .insert(FontFamily::Proportional, proportional.clone());
    fonts.families.insert(slide_family(), proportional);
    Ok(fonts)
}

/// Installs the bundled fonts. On failure egui's default fonts are used for
/// everything (Chinese will not display) and the error is returned so the
/// status bar can show it.
pub fn install(ctx: &egui::Context) -> Result<(), String> {
    let result = find_font_dir()
        .ok_or_else(|| format!("{LATIN_FILE} / {CHINESE_FILE} not found in assets\\fonts"))
        .and_then(|dir| definitions(&dir).map_err(|e| e.to_string()));
    match result {
        Ok(fonts) => {
            ctx.set_fonts(fonts);
            Ok(())
        }
        Err(err) => {
            // Keep the slide family usable with egui's built-in fonts.
            let mut fonts = FontDefinitions::default();
            let defaults = fonts
                .families
                .get(&FontFamily::Proportional)
                .cloned()
                .unwrap_or_default();
            fonts.families.insert(slide_family(), defaults);
            ctx.set_fonts(fonts);
            Err(format!("Fonts missing, Chinese will not display: {err}"))
        }
    }
}
