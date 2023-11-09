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

fn render_diagram() {
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

///Sort all the lines starting with `klant_` in the inhuur_diagram.md file
/// and write back the result to the same file
fn cleanup_diagram() {
    let file = File::open("inhuur_diagram.md").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    // Find blocks of lines starting with `klant_`
    let mut klant_blocks: Vec<(usize, usize)> = Vec::new();
    let mut block_start_idx: Option<usize>= None;
    for (idx, line) in lines.clone().iter().enumerate() {

        if line.starts_with("klant_") {
            if block_start_idx.is_none() {
                block_start_idx = Some(idx);
            }
        } else {
            if block_start_idx.is_some() {
                klant_blocks.push((block_start_idx.unwrap(), idx));
                block_start_idx = None;
            }
        }
    }

    // Sort all the subsections of lines based on the indexes in klant_blocks
    for (start, end) in klant_blocks {
        lines[start..end].sort();
    }

    // Write the lines back to the file
    fs::write("inhuur_diagram.md", lines.join("\n")).unwrap();
}

fn main() {
    cleanup_diagram();
    render_diagram();
}
