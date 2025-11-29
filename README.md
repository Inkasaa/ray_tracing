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

First ball: --ball 0 0.2 1 0 5

0 = x coordinate
0.2 = y coordinate
1 = z coordinate
0 = number_type (0 = Line, like ball #1 in pool)
5 = color/ball index from common.rs (ball #5 = Yellow)

## Custom Scene Arguments

### Balls
`--ball x y z [number_type] [color]`
- x, y, z = position coordinates
- number_type (optional): 0 = Circle, 1 = Line (random if omitted)
- color (optional): 0-16 billiard ball index (random if omitted)

Example: `--ball 0 0.2 1 0 5` → Line ball at (0, 0.2, 1) with Yellow color

### Planes
`--plane px py pz nx ny nz color`
- px, py, pz = point on the plane (x, y, z coordinates)
- nx, ny, nz = normal vector (direction perpendicular to plane surface)
  - Normal vector defines plane orientation and how light bounces off
  - Examples: (0, 1, 0) = horizontal ground, (-1, 0, 0) = vertical wall pointing left, (1, 0, 0) = vertical wall pointing right
- color = 0-16 billiard ball color index

Examples:
- `--plane 0 0 0 0 1 0 7` → Ground plane at y=0 with normal (0,1,0) pointing UP, Maroon color
- `--plane 5 0 0 -1 0 0 1` → Vertical wall at x=5 with normal (-1,0,0) pointing LEFT, Blue color

### Background
`--bg color`
- color = 0-16 billiard ball color index for solid background

Example: `--bg 16` → White background

### Full Example
```bash
./target/release/ray-tracing 1 --custom \
  --plane 0 0 0 0 1 0 7 \
  --ball 0 0.2 1 1 3 \
  --bg 16
```
Creates: Maroon ground plane + Circle green ball + White background

