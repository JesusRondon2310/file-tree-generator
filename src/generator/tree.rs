//! generator/tree.rs

use std::fs;
use std::path::Path;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

pub(super) struct ScanOptions {
    pub(super) max_depth: Option<u32>,
    pub(super) dirs: bool,
    pub(super) visual_look: bool,
}

// 1. Recorre la carpeta actual con las opciones elegidas y escribe el resultado en file_tree.md.
pub(super) fn generate(opts: &ScanOptions) {
    let root = std::env::current_dir().expect("no se pudo leer la carpeta actual");
    let root_name = root.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();

    let mut builder = GitignoreBuilder::new(&root);
    builder.add(root.join(".gitignore"));
    let gi = builder.build().unwrap_or_else(|_| Gitignore::empty());

    let icon = if opts.visual_look { "📦 " } else { "" };
    let root_suffix = if opts.visual_look { "" } else { "/" };
    
    let mut text = format!("{icon}{root_name}{root_suffix}\n");
    build(&root, &gi, "", 1, opts, &mut text);

    let markdown = format!("### Estructura del Proyecto\n\n```text\n{text}```\n");
    fs::write("file_tree.md", markdown).expect("no se pudo escribir file_tree.md");
}

// 2. Arma el árbol de `dir` como texto, respetando `.gitignore` y las opciones elegidas. `.git` nunca se muestra,
// igual que el propio git nunca se sube a sí mismo.
fn build(dir: &Path, gi: &Gitignore, prefix: &str, depth: u32, opts: &ScanOptions, out: &mut String) {
    if opts.max_depth.is_some_and(|max| depth > max) { return; }

    let mut entries: Vec<_> = match fs::read_dir(dir) {
        Ok(read) => read.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|e| e.file_name());

    let entries: Vec<_> = entries
        .into_iter()
        .filter(|e| {
            if e.file_name() == ".git" { return false; }
            
            let path = e.path();
            let is_dir = path.is_dir();
            if opts.dirs && !is_dir { return false; }
            !gi.matched(&path, is_dir).is_ignore()
        })
        .collect();

    let last_index = entries.len().saturating_sub(1);
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == last_index;
        let connector = if is_last { "┗" } else { "┣" };
        let path = entry.path();
        let is_dir = path.is_dir();
        
        let icon = if !opts.visual_look { "" } else if is_dir { "📂 " } else { "📜 " };
        let mut name = entry.file_name().to_string_lossy().to_string();
        
        // Sin emojis, una carpeta se distingue con "/" al final — si no, es indistinguible de un archivo a simple vista.
        if !opts.visual_look && is_dir { name.push('/'); }
        out.push_str(&format!("{prefix} {connector} {icon}{name}\n"));

        if is_dir {
            let branch = if is_last { "  " } else { "┃ " };
            let new_prefix = format!("{prefix} {branch}");
            build(&path, gi, &new_prefix, depth + 1, opts, out);
        }
    }
}
