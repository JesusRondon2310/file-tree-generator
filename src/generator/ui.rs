//! generator/ui.rs

use std::io::{self, Write};

use super::tree::ScanOptions;

fn ask(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().to_string()
}

fn ask_bool(header: &str, opt0: &str, opt1: &str) -> bool {
    loop {
        println!("{header}");
        println!("      0 = {opt0}");
        println!("      1 = {opt1}");
        match ask("> Opción: ").as_str() {
            "0" => return false,
            "1" => return true,
            _ => println!("Opción inválida, intenta de nuevo.\n"),
        }
    }
}

// ask_depth: [Enter] -> sin límite (None); un entero positivo (0 >) -> ese límite; cualquier otra cosa se rechaza.
fn ask_depth() -> Option<u32> {
    loop {
        println!("[3/3] Límite de profundidad del árbol:");
        println!("      [Enter] para escanear sin límite, o escribe un número positivo (ej. 3)");
        let input = ask("> Nivel de profundidad: ");
        if input.is_empty(){
            return None;
        }
        match input.parse::<u32>() {
            Ok(n) if n > 0 => return Some(n),
            _ => println!("Opción inválida, intenta de nuevo.\n"),
        }
    }
}

// 1. Muestra el encabezado y hace las 3 preguntas, hasta reunir las opciones de escaneo completas.
pub(super) fn ask_options() -> ScanOptions {
    println!("==================================================");
    println!(" 📦 FILE TREE GENERATOR ");
    println!("==================================================\n");

    let dirs = ask_bool(
        "[1/3] Selecciona el modo de escaneo:",
        "Carpetas y archivos (Completo)",
        "Solo carpetas",
    );
    println!();

    let visual_look = ask_bool(
        "[2/3] Estilo visual del árbol:",
        "Con emojis (📦, 📂, 📜)",
        "Texto plano clásico (Ramas puras)",
    );
    println!();

    let max_depth = ask_depth();
    println!();

    ScanOptions { max_depth, dirs, visual_look: !visual_look }
}

// 2. Avisa que empezó a generar el árbol.
pub(super) fn announce_generating() {
    println!("[✓] Generando estructura...");
}

// 3. Avisa que el archivo quedó listo.
pub(super) fn done() {
    println!("[✓] ¡Éxito! Archivo 'file_tree.md' creado.");
}

// 4. Espera a que el usuario presione Enter antes de cerrar la consola.
pub(super) fn wait_exit() {
    ask("Presiona [Enter] para salir...");
}
