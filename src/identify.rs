use toml::Table;
use tracing::{info, error};

use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct ProjectSchema {
    project_name: String,
    author: String,
    description: String,
    //
    has_dockerfile: bool,
    has_toml_config: bool,
    root_location: String,
}

impl ProjectSchema {
    fn default() -> Self {
        Self {
            project_name: "".to_string(),
            author: "".to_string(),
            description: "".to_string(),
            has_dockerfile: false,
            has_toml_config: true, // We check to see if it needs to be false so it's true here
            root_location: "".to_string(),
        }
    }
}

pub fn identify(file_location: String) -> ProjectSchema {
    let mut output = ProjectSchema::default();

    // Does this project not have a Shuttle.toml file?
    if !Path::new(format!("{}{}", file_location, "/Shuttle.toml").as_str()).exists() {
        output.has_toml_config = false;
    } else {
        match fs::read_to_string(format!("{}{}", file_location, "/Shuttle.toml").as_str()) {
            Ok(data) => {
                match data.parse::<Table>() {
                    Ok(toml) => {
                        output.project_name = toml["name"].as_str().unwrap_or("NO_NAME").to_string();
                        output.author = toml["author"].as_str().unwrap_or("NO_AUTHOR").to_string();
                        output.description = toml["description"].as_str().unwrap_or("NO_DESCRIPTION").to_string();
                    },
                    Err(e) =>{
                        error!("Failed to parse Shuttle.toml: {}" , e);
                    }
                }
            },
            Err(e) => {
                error!("Failed to open Shuttle.toml file!: {}" , e);
            }
        }
    }
    
    // Does the project have a preconfigured dockerfile to use?
    if Path::new(format!("{}{}", file_location, "/Dockerfile").as_str()).exists() {
        output.has_dockerfile = true; 
        // We can assume that the dockerfile is preconfigured to work properly with containers
        // so we can just immediately execute it.
    }

    println!("{}", format!("{output:?}"));

    output.root_location = file_location;

    output
}