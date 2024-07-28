cargo run --release -- -i assets/cube.obj -o results/cube_1.obj -c 1
cargo run --release -- -i results/cube_1.obj -o results/cube_2.obj -c 1
cargo run --release -- -i results/cube_2.obj -o results/cube_3.obj -c 1

cargo run --release -- -i assets/triangulated_cube.obj -o results/triangulated_cube_1.obj -c 1
cargo run --release -- -i results/triangulated_cube_1.obj -o results/triangulated_cube_2.obj -c 1
cargo run --release -- -i results/triangulated_cube_2.obj -o results/triangulated_cube_3.obj -c 1

cargo run --release -- -i assets/pyramid.obj -o results/pyramid_1.obj -c 1
cargo run --release -- -i results/pyramid_1.obj -o results/pyramid_2.obj -c 1
cargo run --release -- -i results/pyramid_2.obj -o results/pyramid_3.obj -c 1

cargo run --release -- -i assets/single_plane.obj -o results/single_plane_1.obj -c 1
cargo run --release -- -i assets/single_point.obj -o results/single_point_1.obj -c 1
cargo run --release -- -i assets/3_edges.obj -o results/3_edges_1.obj -c 1
cargo run --release -- -i assets/two_different_directed_planes.obj -o results/two_different_directed_planes_1.obj -c 1

cargo run --release -- -i assets/cessna.obj -o results/cessna_1.obj -c 1
cargo run --release -- -i results/cessna_1.obj -o results/cessna_2.obj -c 1
cargo run --release -- -i results/cessna_2.obj -o results/cessna_3.obj -c 1
