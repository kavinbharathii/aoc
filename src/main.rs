#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

// ====================================================== Webdev ====================================================== //

fn aoc(project_name: &str) {
    if !Path::new(project_name).exists() {
        let _dire = fs::create_dir(project_name).expect(format!("Error encountered while creating project: {}!", project_name).as_str());
    } else {
        println!("{} already exists", project_name);
        process::exit(0);
    }

    // ------------------------------------------------ html ------------------------------------------------ // 
    let mut mainpy = File::create(format!("{}/{}", project_name, "main.py")).expect("Failed to create main.py");
    mainpy.write_all(format!(r#"

# Advent Of Code: {}

with open("data.aoc", "r") as file:
    raw_data = [line.strip() for line in file.readlines()]
    file.close()

"#, project_name).as_bytes()).expect("Error while writing to index.html");

    let dataaoc = File::create(format!("{}/{}", project_name, "data.aoc")).expect("Failed to create data.aoc");
    let testaoc = File::create(format!("{}/{}", project_name, "test.aoc")).expect("Failed to create test.aoc");
}
    
fn main() {

    match std::env::args().nth(1) {
        Some(project_name) => {
            aoc(&project_name)
        },

        None => {
            println!("No project name given");
            process::exit(0);
        }
    }
}
