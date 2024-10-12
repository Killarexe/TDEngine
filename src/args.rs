use std::path::PathBuf;

use clap::Parser;

/// 3D model viewer/renderer on the terminal.
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    ///Target OBJ file to render (by default it's a cube).
    pub obj_file_path: Option<PathBuf>
}
