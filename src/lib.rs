//  dP""b8  dP"Yb  8888b.   dP"Yb  888888          888888 Yb  dP 88""Yb .dP"Y8 888888 
// dP   `" dP   Yb  8I  Yb dP   Yb   88   ________   88    YbdP  88__dP `Ybo."   88   
// Yb  "88 Yb   dP  8I  dY Yb   dP   88   """"""""   88     8P   88"""  o.`Y8b   88   
//  YboodP  YbodP  8888Y"   YbodP    88              88    dP    88     8bodP'   88   
//
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::env;
use godot::prelude::*;
use godot::engine::{Sprite2D, ISprite2D};
use tempfile::tempdir;

#[derive(GodotClass)]
#[class(base = Sprite2D)]
pub struct Typst {
    #[base]
    pub node: Base<Sprite2D>,
    #[export]
    pub typst_expression: GString,
}

#[godot_api]
impl ISprite2D for Typst {
    fn init(node: Base<Sprite2D>) -> Self {
        Typst { 
            node,
            typst_expression: String::new().into(),
        }
    }

    fn ready(&mut self) {
        self.render();
    }

    fn process(&mut self, delta: f64) {}
}

#[godot_api]
impl Typst {
    pub fn render(&mut self) {
        // Render Typst: convert latex expression to SVG, then assign to self
        // Step 1: Create a temporary .typst file
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join("expression.typst");
        let mut file = File::create(&file_path)
            .expect("Failed to create .typst file");

        writeln!(file, "{}", self.typst_expression)
            .expect("Failed to write to .typst file");

        // Step 2: Execute 'typst compile'
        let output_path = dir.path().join("output.svg");
        let status = Command::new("typst")
            .arg("compile")
            .arg(&file_path)
            .arg("--format")
            .arg("svg")
            .arg("--output")
            .arg(&output_path)
            .status()
            .expect("Failed to execute typst command");

        if !status.success() {
            eprintln!("Error: Typst command failed");
            return;
        }

        // At this point, 'output.svg' contains the compiled SVG.
        // You can proceed to load this as a Godot Texture.
    }
}
