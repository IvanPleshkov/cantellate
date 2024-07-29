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

Also, docker image is available:
```bash
docker run --rm --volume "${PWD}:/data" pleshkov:cantellation ./cantellation -i data/assets/cube.obj -o data/results/cube_cantellated.obj
```

# Assets
There are some example meshes in the `assets` directory. Example contains:
- Simple cube. Basic example of cantellation
- Triangulated cube. All faces are triangles
- Pyramid and torus
- Generic 3D models: cessna and suzanne
- Single plane
- Plane mesh to test empty gaps
- Mesh contains the edge with 3 connected faces
- Single point mesh
- Mesh with connected faces but with opposite normals
- Mesh with face duplications (open problem, see below)

All assests are in the `.obj` format. Cantellated meshes are saved in the `results` directory.

# Open problems
This solution has open problems:
- Many iterations of cantellation for the cube produces non-convex mesh. The algorithm should be improved to handle this case. The problem is caused by the fact that the new face on iteration 2 produces a face which is not convex. The proper solution is to split the face into convex parts by polygon triangulation. It will produce more faces but the mesh will be convex.
- Mesh with face duplications (two faces that have the same vertex indices). The algorithm does not handle the case when the face is duplicated. The algorithm should be improved to handle this case. Currently it just skips some faces.

# Future improvements
There are some intresting subjects to improve:
- Add support for other file formats
- Better result for non-convex meshes. By definition of cantellation, the faces are pushed outwards. For non-convex meshes, new faces intersect with each other. It's interesting to handle this case and do mesh intersection. Or at least reduce size of result faces to avoid self intersections for small factor.
- Parallel processing. The algorithm is simple and can be parallelized. There is a branch of experiment with `rayon` crate but it's not finished yet (cause it does not show acceptable performance result).
