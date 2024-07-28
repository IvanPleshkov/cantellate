mod cantellate;
mod mesh;
mod vec3;

use std::path::PathBuf;

use cantellate::cantellate;
use clap::Parser;
use mesh::Mesh;
use num_traits::{Float, FromPrimitive, ToPrimitive};

/// Command line arguments.
#[derive(Parser, Debug, Clone)]
#[clap(about)]
pub struct Args {
    /// Input `.obj` file.
    /// If input is a directory, all `.obj` files in the directory will be processed.
    #[arg(short, long)]
    input: String,

    /// Output `.obj` file.
    /// If input is a directory, all output files will be saved in this directory.
    #[arg(short, long)]
    output: String,

    /// Cantellation factor.
    #[clap(short, long, default_value_t = 1.0)]
    factor: f32,

    /// Epsilon value for floating point comparison.
    #[clap(short, long, default_value_t = 0.001)]
    epsilon: f32,

    /// Count of cantellation iterations.
    #[clap(short, long, default_value_t = 1)]
    count: usize,

    /// Use double precision.
    #[clap(short, long, default_value_t = true)]
    double: bool,
}

fn main() {
    // parse command line arguments
    let args = Args::parse();

    let input_path: PathBuf = args.input.clone().into();
    if input_path.is_file() {
        if args.double {
            run::<f64>(&args);
        } else {
            run::<f32>(&args);
        }
    } else if input_path.is_dir() {
        let output_dir = PathBuf::from(&args.output);
        if output_dir.exists() {
            std::fs::remove_dir_all(&args.output).unwrap();
        }

        // if input is a directory, iterate over all files in the directory
        for file in input_path.read_dir().unwrap() {
            let file = file.unwrap();
            if file.path().is_file()
                && file.path().extension().and_then(|e| e.to_str()) == Some("obj")
            {
                let input = file.path().to_str().unwrap().to_string();
                let output = output_dir
                    .join(file.file_name())
                    .to_str()
                    .unwrap()
                    .to_string();
                let args = Args {
                    input,
                    output,
                    ..args.clone()
                };
                if args.double {
                    run::<f64>(&args);
                } else {
                    run::<f32>(&args);
                }
            }
        }
    }
}

// Run the demo.
fn run<N>(args: &Args)
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
    println!("Input mesh: {}, output {}", args.input, args.output);
    // load the input mesh
    let mesh = Mesh::<N>::load_obj(&args.input).unwrap();

    // do the cantellation
    let output_mesh = (0..args.count).fold(mesh, |mesh, iteration| {
        let timer = std::time::Instant::now();
        let result = cantellate(
            &mesh,
            N::from_f32(args.factor).unwrap(),
            N::from_f32(args.epsilon).unwrap(),
        );
        println!(
            "Iteration {} took {:?}; vertices count: {}",
            iteration,
            timer.elapsed(),
            result.vertices.len()
        );
        result
    });

    let output_path: PathBuf = args.output.clone().into();
    if let Some(output_dir) = output_path.parent() {
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).unwrap();
        }
    }

    // save the output mesh
    output_mesh.save_obj(&args.output).unwrap();
}
