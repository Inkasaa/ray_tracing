# Rust Ray Tracer


This is a compact educational ray tracer in one source file that outputs an ASCII PPM image to stdout.


## Build & run


```bash
cargo build --release
# produce an image (scene 3, 800x600, recursion depth 3)
./target/release/ray-tracing 3 800 600 3 > output.ppm
```


## Scenes
1 - single sphere
2 - plane + cube (lower brightness)
3 - all objects (sphere, cube, cylinder, plane)
4 - same as 3 but different camera position


## Toggle parameters
- argument 1: 
- argument 2: 
- argument 3: 
- argument 4: 