pub mod nest;
pub mod react;

use std::path::PathBuf;

pub trait ProjectBuilder {
    fn build(&self, name: &str, base_path: PathBuf, package_path: PathBuf) -> Result<ExitStatus, std::io::Error> ;
}