// src/lib.rs
/*
 * Core library for GrantProgram
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Custom result type for the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Process result struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    /// Success flag
    pub success: bool,
    /// Message describing the result
    pub message: String,
    /// Optional data associated with the result
    pub data: Option<serde_json::Value>,
}

/// Grant program processor
#[derive(Debug)]
pub struct GrantProgramProcessor {
    /// Flag for verbose mode
    pub verbose: bool,
    /// Count of processed items
    pub processed_count: usize,
}

impl GrantProgramProcessor {
    /// Create a new processor instance
    /// 
    /// # Arguments
    /// 
    /// * `verbose` - Flag to enable verbose mode
    /// 
    /// # Returns
    /// 
    /// A new `GrantProgramProcessor` instance
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Process the given data
    /// 
    /// # Arguments
    /// 
    /// * `data` - Data to process
    /// 
    /// # Returns
    /// 
    /// A `ProcessResult` instance describing the result of processing
    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    /// Get statistics about the processor
    /// 
    /// # Returns
    /// 
    /// A JSON value containing the processor statistics
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    // Initialize logging
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    
    info!("Starting GrantProgram processing");
    
    let mut processor = GrantProgramProcessor::new(verbose);
    
    // Read input
    let input_data = match input {
        Some(path) => {
            // Read file at the given path
            fs::read_to_string(path).map_err(|e| e.into())
        }
        None => {
            // Use default input (e.g., stdin)
            Ok(String::new())
        }
    }?;

    // Process the input data
    processor.process(&input_data)?;

    // Write output
    let output_data = serde_json::to_string(&processor.get_stats())?;
    if let Some(output_path) = output {
        fs::write(output_path, output_data)?;
    } else {
        println!("{}", output_data);
    }

    Ok(())
}