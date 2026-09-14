//! generator/core.rs

use super::{ui, tree};

// 1. Pregunta las opciones, genera el árbol, y avisa cuando termina.
pub fn run() {
    let opts = ui::ask_options();
    ui::announce_generating();
    tree::generate(&opts);
    ui::done();
    ui::wait_exit();
}
