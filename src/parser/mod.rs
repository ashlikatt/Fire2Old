pub mod program_structure;

pub mod parser {
    use std::{collections::HashMap, path::Path, fs};

    use crate::{resources::ResourceLoc, compiler::CompileError};

    use super::program_structure::Program;
    

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
        let mut program = Program::new();
        let location = ResourceLoc::new();
        
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
            parse_folder(&mut program, location.with(f.file_name().to_str().unwrap().to_string()), &f.path());
        }
        todo!();
    }

    pub fn parse_folder<'a>(program: &'a mut Program, loc: ResourceLoc, path: &Path) -> Result<(), CompileError<'a>> {
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
                parse_folder(program, loc.with(f.file_name().to_str().unwrap().to_string()), &f.path());
            } else if ftype.is_file() {
                parse_file(program, loc.with(f.file_name().to_str().unwrap().to_string()), &f.path());
            } else {
                panic!("Error!");
            }
        }
        Ok(())
    }

    pub fn parse_file<'a>(program: &'a mut Program, loc: ResourceLoc, path: &Path) -> Result<(), CompileError<'a>> {
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
}