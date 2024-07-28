In the geometry of polytopes, the procedure of expansion or cantellation describes a procedure where edges in 2D or faces in 3D are disconnected and pushed outwards (in the direction of the edge or face normal) from the center of a body. The gaps are filled with new edges or faces to create a new polytope.

This crate implements the cantellation of polytopes in 3D for wavefront `.obj` files.

# Usage
Run help command to see the available options:
```bash
cargo run --release -- --help
```
With output:
```
Usage: cantellation [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>      Input `.obj` file. If input is a directory, all `.obj` files in the directory will be processed
  -o, --output <OUTPUT>    Output `.obj` file. If input is a directory, all output files will be saved in this directory
  -f, --factor <FACTOR>    Cantellation factor [default: 1]
  -e, --epsilon <EPSILON>  Epsilon value for floating point comparison [default: 0.001]
  -c, --count <COUNT>      Count of cantellation iterations [default: 1]
  -d, --double             Use double precision
  -h, --help               Print help
```

For instance, cube cantellation with factor 1:
```bash
cargo run --release -- -i assets/cube.obj -o results/cube_cantellated.obj
```

Two iterations:
```bash
cargo run --release -- -i assets/cube.obj -o results/cube_cantellated.obj -c 2
```
