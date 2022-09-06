use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct InputData {
    /// Name of the image to view
    file_name: String,
}

impl InputData {
    pub fn file_name(&self) -> &str {
        &self.file_name
    }
}