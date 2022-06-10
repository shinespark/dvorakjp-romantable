use std::path::PathBuf;

pub struct RomanTableWithEmojiBuilder {}

impl RomanTableWithEmojiBuilder {
    pub fn exec(
        emoji_file: PathBuf,
        output_file: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("{:?}", emoji_file);
        println!("{:?}", output_file);
        Ok(())
    }
}
