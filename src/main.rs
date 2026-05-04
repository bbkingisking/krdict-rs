mod api;
mod cli;
mod parser;
mod types;

use clap::Parser;
use cli::{Cli, Commands};
use quick_xml::de::from_str;
use std::fs;
use types::{ApiError, SearchChannel};

fn main() {
    let cli = Cli::parse();

    // Get API key
    let api_key = match api::get_api_key() {
        Ok(key) => key,
        Err(e) => {
            if !cli.quiet {
                eprintln!("Error: {}", e);
            }
            std::process::exit(1);
        }
    };

    // Build URL based on command
    let url = match &cli.command {
        Commands::Search { .. } => api::build_search_url(&api_key, &cli.command),
        Commands::View { .. } => api::build_view_url(&api_key, &cli.command),
    };

    let url = match url {
        Ok(u) => u,
        Err(e) => {
            if !cli.quiet {
                eprintln!("Error building URL: {}", e);
            }
            std::process::exit(1);
        }
    };

    // Make request
    let response = match api::make_request(&url) {
        Ok(r) => r,
        Err(e) => {
            if !cli.quiet {
                eprintln!("Error making request: {}", e);
            }
            std::process::exit(1);
        }
    };

    // Check for error response
    if response.contains("<error>") {
        match from_str::<ApiError>(&response) {
            Ok(error) => {
                if !cli.quiet {
                    eprintln!("API Error [{}]: {}", error.error_code, error.message);
                }
                std::process::exit(1);
            }
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Failed to parse error response: {}", e);
                    eprintln!("Raw response: {}", response);
                }
                std::process::exit(1);
            }
        }
    }

    // Parse and display results based on command type
    match &cli.command {
        Commands::Search { .. } => {
            match from_str::<SearchChannel>(&response) {
                Ok(channel) => {
                    // Serialize to JSON
                    match serde_json::to_string_pretty(&channel) {
                        Ok(json) => {
                            // Print to stdout
                            if !cli.quiet {
                                println!("{}", json);
                            }
                            
                            // Export to file if requested
                            if let Some(export_path) = &cli.export {
                                if let Err(e) = fs::write(export_path, &json) {
                                    if !cli.quiet {
                                        eprintln!("Failed to write JSON to file: {}", e);
                                    }
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            if !cli.quiet {
                                eprintln!("Failed to serialize to JSON: {}", e);
                            }
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    if !cli.quiet {
                        eprintln!("Failed to parse search results: {}", e);
                        eprintln!("Raw response: {}", response);
                    }
                    std::process::exit(1);
                }
            }
        }
        Commands::View { .. } => {
            // Use roxmltree parser to handle duplicate pos tags
            match parser::parse_view_response(&response) {
                Ok(channel) => {
                    // Serialize to JSON
                    match serde_json::to_string_pretty(&channel) {
                        Ok(json) => {
                            // Print to stdout
                            if !cli.quiet {
                                println!("{}", json);
                            }
                            
                            // Export to file if requested
                            if let Some(export_path) = &cli.export {
                                if let Err(e) = fs::write(export_path, &json) {
                                    if !cli.quiet {
                                        eprintln!("Failed to write JSON to file: {}", e);
                                    }
                                    std::process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            if !cli.quiet {
                                eprintln!("Failed to serialize to JSON: {}", e);
                            }
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    if !cli.quiet {
                        eprintln!("Failed to parse view results: {}", e);
                        eprintln!("Raw response: {}", response);
                    }
                    std::process::exit(1);
                }
            }
        }
    }
}