//! Generates `src/generated_case_studies.rs` from every `.md` file under
//! `../docs/cse_studies/` so a new case study only needs a new markdown file
//! — no Rust code change. Mirrors the same pattern used by the shalgo blog.
#![allow(
    clippy::print_stdout,
    reason = "println! is how a build script talks to Cargo (cargo:rerun-if-changed, cargo:warning)"
)]

use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let case_studies_dir = Path::new("../docs/cse_studies");
    println!("cargo:rerun-if-changed=../docs/cse_studies");

    let mut slugs = Vec::new();
    if case_studies_dir.is_dir() {
        for entry in fs::read_dir(case_studies_dir)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Some(slug) = path.file_stem().and_then(|s| s.to_str()) {
                    slugs.push(slug.to_string());
                }
            }
        }
    }
    slugs.sort();

    let mut out = String::new();
    for slug in &slugs {
        let const_name = slug.to_uppercase().replace('-', "_");
        writeln!(
            out,
            "const CASE_STUDY_{const_name}: &str = include_str!(\"../../docs/cse_studies/{slug}.md\");"
        )?;
    }

    writeln!(out)?;
    writeln!(out, "/// Load a case study's raw markdown by slug.")?;
    writeln!(
        out,
        "pub fn load_case_study(slug: &str) -> Option<&'static str> {{"
    )?;
    writeln!(out, "    match slug {{")?;
    for slug in &slugs {
        let const_name = slug.to_uppercase().replace('-', "_");
        writeln!(out, "        \"{slug}\" => Some(CASE_STUDY_{const_name}),")?;
    }
    writeln!(out, "        _ => None,")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;
    writeln!(out)?;

    writeln!(out, "/// All known case study slugs.")?;
    writeln!(
        out,
        "pub fn list_case_study_slugs() -> &'static [&'static str] {{"
    )?;
    write!(out, "    &[")?;
    for slug in &slugs {
        write!(out, "\"{slug}\", ")?;
    }
    writeln!(out, "]")?;
    writeln!(out, "}}")?;

    let dest_path = Path::new("src/generated_case_studies.rs");
    let mut file = fs::File::create(dest_path)?;
    file.write_all(out.as_bytes())?;

    Ok(())
}
