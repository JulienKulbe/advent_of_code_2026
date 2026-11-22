use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub enum Input {
    Small,
    Large,
}

pub fn read_file(base_path: &Path, input: Input) -> Result<BufReader<File>> {
    let file_name = match input {
        Input::Small => "input_small.txt",
        Input::Large => "input.txt",
    };
    let file = File::open(base_path.join(file_name))?;
    Ok(BufReader::new(file))
}

