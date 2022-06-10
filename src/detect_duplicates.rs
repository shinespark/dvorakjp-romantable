use std::path::PathBuf;

pub struct DuplicateDetector {}

impl DuplicateDetector {
    pub fn exec(detect_file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", detect_file);
        Ok(())
    }
}
