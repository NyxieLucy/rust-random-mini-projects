use std::fs;
use std::path::{Path, PathBuf};
use pulldown_cmark::html;

mod error;
use error::{Error, Result};

// Rendering rendering markdown
fn render_markdown(input: &str) -> String {
    let mut html_output = String::new();
    let parser = pulldown_cmark::Parser::new(input);
    html::push_html(&mut html_output, parser);
    html_output
}

fn build_file(src_path: &Path, dist_dir: &Path) -> Result<()> {
    // ensures dist path exists
    let dest_path = dist_dir
        .join(
            src_path
                .file_stem()
                .ok_or_else(|| Error::InvalidPath(src_path.to_owned()))?,
        )
        .with_extension("html");

    let content = fs::read_to_string(src_path)
        .map_err(|e| Error::Io {
            path: src_path.to_owned(),
            source: e,
        })?;

    let html = render_markdown(&content);

    fs::write(&dest_path, html).map_err(|e| Error::Io {
        path: dest_path,
        source: e,
    })?;

    Ok(())
}

fn process_content_dir(content_dir: &Path, dist_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(content_dir).map_err(|e| Error::Io {
        path: content_dir.to_owned(),
        source: e,
    })? {
        let entry = entry.map_err(|e| Error::Io {
            path: content_dir.to_owned(),
            source: e,
        })?;

        let path = entry.path();

        // Only process .md files
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        build_file(&path, dist_dir)?;
    }

    Ok(())
}

fn setup_dirs(dist_dir: &Path) -> Result<()> {
    fs::create_dir_all(dist_dir).map_err(|e| Error::Io {
        path: dist_dir.to_owned(),
        source: e,
    })?;
    Ok(())
}

fn main() -> Result<()> {
    let content_dir = Path::new("content");
    let dist_dir = Path::new("dist");

    setup_dirs(dist_dir)?;
    process_content_dir(content_dir, dist_dir)?;

    println!("Site generated successfully!");
    Ok(())
}
