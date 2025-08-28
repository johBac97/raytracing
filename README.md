# Raytracing

A simple terminal-based raycasting demo written in Rust, rendering a 3D perspective view from a 2D map.

## How to Run

1. Ensure Rust is installed (`rustc` and `cargo`).
2. Clone the repository.
3. Run the program with a map file:
   ```bash
   cargo run -- data/maps/<map_file>
   ```
   Replace `<map_file>` with a map file from the `data/maps/` directory.

## Controls
- `W`: Move forward
- `S`: Move backward
- `A`: Rotate left
- `D`: Rotate right
- `P`: Print debug info
- `Q`: Quit