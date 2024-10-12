use std::{fs::File, io::BufReader};

use clap::Parser;
use obj::Obj;
use tdengine::{args::Args, engine::Engine, vec::{Vector2, Vector3}};

fn main() {
    let args: Args = Args::parse();
    let mut engine: Engine = Engine::new();
    engine.canvas.verticies = vec![
        (-10.0, -10.0, -10.0).into(),
        (-10.0, -10.0, 10.0).into(),
        (10.0, -10.0, 10.0).into(),
        (10.0, -10.0, -10.0).into(),
        (-10.0, 10.0, -10.0).into(),
        (-10.0, 10.0, 10.0).into(),
        (10.0, 10.0, 10.0).into(),
        (10.0, 10.0, -10.0).into(),
    ];
    engine.canvas.indicies = vec![
        (0, 1).into(),
        (1, 2).into(),
        (2, 3).into(),
        (3, 0).into(),
        (4, 5).into(),
        (5, 6).into(),
        (6, 7).into(),
        (7, 4).into(),
        (0, 4).into(),
        (1, 5).into(),
        (2, 6).into(),
        (3, 7).into(),
    ];
    if let Some(obj_path) = args.obj_file_path {
        if let Ok(obj_file) = File::open(obj_path) {
            let buffer: BufReader<File> = BufReader::new(obj_file);
            let obj: Obj = obj::load_obj(buffer).expect("Tried to load non OBJ file.");
            engine.canvas.verticies = obj.vertices.iter().map(|v| Vector3::new(v.position[0], v.position[1], v.position[2])).collect();
            engine.canvas.indicies = obj.indices.windows(2).map(|i| Vector2::new(i[0] as usize, i[1] as usize)).collect();
        }
    }
    engine.run().unwrap();
}
