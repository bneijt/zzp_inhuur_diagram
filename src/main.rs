use tera::{Tera, Context};
use std::fs::File;
use std::io::{self, BufRead};
use std::fs;
use std::path::Path;


// Render templates in templates using tera
fn render_template(diagram: String) {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = Context::new();
    context.insert("diagram", &diagram);
    let rendered = tera.render("index.html", &context).unwrap();

    if !Path::new("public").exists() {
        fs::create_dir_all("public").unwrap();
    }
    fs::write("public/index.html", rendered).unwrap();
}
fn main() {
    let file = File::open("inhuur_diagram.md").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Drop the first and last line
    lines.remove(0);
    lines.pop();

    // Convert the lines back into a single string
    let diagram: String = lines.join("\n");

    // Pass the string to the render_template function
    render_template(diagram);
}
