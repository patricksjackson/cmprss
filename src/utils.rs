use std::io;
use std::path::Path;

pub struct CmprssCommonArgs {
    pub compress: bool,
    pub extract: bool,
    pub input: Option<String>,
    pub output: Option<String>,
}

/// Common interface for all compressor implementations
#[allow(unused_variables)]
pub trait Compressor {
    /// Name of this Compressor
    fn name(&self) -> &str;

    /// Default extension for this Compressor
    fn extension(&self) -> &str {
        self.name()
    }

    /// Generate the default name for the compressed file
    fn default_compressed_filename(&self, in_path: &Path) -> String {
        format!(
            "{}.{}",
            in_path.file_name().unwrap().to_str().unwrap(),
            self.extension()
        )
    }

    // Generate the default extracted filename
    fn default_extracted_filename(&self, in_path: &Path) -> String {
        in_path.file_stem().unwrap().to_str().unwrap().to_string()
    }

    /// Getter method for the common arguments
    // TODO: There is probably a cleaner way to do this?
    fn common_args(&self) -> &CmprssCommonArgs;

    fn compress(&self, input: CmprssInput, output: CmprssOutput) -> Result<(), io::Error> {
        cmprss_error("compress_target unimplemented")
    }

    fn extract(&self, input: CmprssInput, output: CmprssOutput) -> Result<(), io::Error> {
        cmprss_error("extract_target unimplemented")
    }
}

pub fn cmprss_error(message: &str) -> Result<(), io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, message))
}

/// Defines the possible inputs of a compressor
// TODO: Implement fmt for CmprssInput/CmprssOutput
pub enum CmprssInput<'a> {
    /// Path(s) to the input files.
    Path(Vec<&'a Path>),
    /// Input pipe
    Pipe(std::io::Stdin),
}

/// Defines the possible outputs of a compressor
pub enum CmprssOutput<'a> {
    Path(&'a Path),
    Pipe(std::io::Stdout),
}
