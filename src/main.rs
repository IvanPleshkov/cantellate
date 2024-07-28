mod cantellate;
mod mesh;
mod vec3;

use cantellate::cantellate;
use clap::Parser;
use mesh::Mesh;
use num_traits::{Float, FromPrimitive, ToPrimitive};

/// Command line arguments.
#[derive(Parser, Debug, Clone)]
#[clap(about)]
pub struct Args {
    /// Input `.obj` file.
    #[arg(short, long)]
    input: String,

    /// Output `.obj` file.
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

    if args.double {
        run::<f64>(args);
    } else {
        run::<f32>(args);
    }
}

// Run the demo.
fn run<N>(args: Args)
where
    N: Float + ToPrimitive + FromPrimitive + Default,
{
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

    // save the output mesh
    output_mesh.save_obj(&args.output).unwrap();
}
