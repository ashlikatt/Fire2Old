use std::{collections::HashSet, path::Path, fs};

use crate::{compiler::CompileError, parser_traits::{Program, ProgramResources}};

const STD_LIB: &str = "stdlib";

pub fn parse_project(path: &Path) -> Result<Program, CompileError> {
    // Validate data
    if !path.exists() {
        panic!("Directory doesn't exist"); // Invalid
    }
    if !path.is_dir() {
        panic!("Must be directory"); // Invalid
    }

    // Create the resource container
    let mut resources = ProgramResources { resources: HashSet::new() };
    
    // Get the children, these will be packages
    let children = fs::read_dir(path).unwrap(); // Shouldn't error because of prev. two checks.

    // Always add the standard library to packages
    let std = Path::new(STD_LIB);
    let packages = children.chain(fs::read_dir(std).unwrap());

    // Loop over packages and add relevant data
    for package in packages {
        let f = package.unwrap_or_else(|_| { panic!("Error!") });
        let ftype = f.file_type().unwrap_or_else(|_| { panic!("Error!") });
        if !ftype.is_dir() { panic!("Must be directory"); }
        parse_folder(&mut resources, &f.path());
    }
    todo!();
}

pub fn parse_folder(res: &mut ProgramResources, path: &Path) -> Result<(), CompileError> {
    // Validate data
    if !path.exists() {
        panic!("Directory doesn't exist"); // Invalid
    }
    if !path.is_dir() {
        panic!("Must be directory"); // Invalid
    }
    
    // Get the children, these will be packages
    let children = fs::read_dir(path).unwrap(); // Shouldn't error because of prev. two checks.

    // Loop over packages and add relevant data
    for file in children {
        let f = file.unwrap_or_else(|_| { panic!("Error!") });
        let ftype = f.file_type().unwrap_or_else(|_| { panic!("Error!") });
        if ftype.is_dir() {
            parse_folder(res, &f.path());
        } else if ftype.is_file() {
            parse_file(res, &f.path());
        } else {
            panic!("Error!");
        }
    }
    Ok(())
}

pub fn parse_file(res: &mut ProgramResources, path: &Path) -> Result<(), CompileError> {
    // Validate data
    if !path.exists() {
        panic!("File doesn't exist"); // Invalid
    }
    if !path.is_file() {
        panic!("Must be file"); // Invalid
    }
    if let Some(b) = path.extension() {
        if b != "fire" {
            panic!("Must be a fire file"); // Invalid
        }
    }
    println!("{:?} {:?}", path.file_stem().unwrap(), path.extension().unwrap());
    Ok(())
}