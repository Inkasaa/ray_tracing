# Rust Ray Tracer


This is a compact educational ray tracer in one source file that outputs an ASCII PPM image to stdout.


## Build & run


```bash
cargo build --release
# produce an image (scene 3, 800x600, recursion depth 3)
./target/release/ray-tracing > output.ppm
```


## Scenes
1 - single sphere
2 - plane + cube (lower brightness)
3 - all objects (sphere, cube, cylinder, plane)
4 - same as 3 but different camera position



First ball: --ball 0 0.2 1 0 5

0 = x coordinate
0.2 = y coordinate
1 = z coordinate
0 = number_type (0 = Line, like ball #1 in pool)
5 = color/ball index from common.rs (ball #5 = Yellow)

## Custom Scene Arguments

### Balls
`--ball x y z [0 or 1] [color]`

- x, y, z = position coordinates
-0 = Circle, 1 = Line (random if omitted)
- color (optional): 0-16 billiard ball index (random if omitted)

Example: `--ball 0 0.2 1 0 5` → Line ball at (0, 0.2, 1) with Yellow color

<!-- The old --plane flag has been deprecated in favor of the simpler --ground convenience flag. -->

### Ground (convenience)
`--ground [--noise] [color]`
- Creates a horizontal ground plane at the origin (0,0,0) with normal (0,1,0)
- By default (no arguments), uses Perlin noise with tournament green felt
- `--noise` flag enables Perlin noise texture
- `color` (optional) = 0-9 billiard table felt color index:
  - 0: Tournament Green (default)
  - 1: Dark Green (traditional)
  - 2: English Green
  - 3: Navy Blue
  - 4: Electric Blue
  - 5: Burgundy/Wine Red
  - 6: Dark Red
  - 7: Camel/Tan (vintage)
  - 8: Charcoal Grey
  - 9: Purple (modern)
- Without `--noise` and with a color index, creates a solid colored felt

Examples:
- `--ground` → Perlin noise ground with default tournament green
- `--ground 3` → Solid navy blue felt (no noise)
- `--ground --noise 3` → Navy blue felt with Perlin noise texture
- `--ground --noise 5` → Burgundy felt with Perlin noise texture
- `--ground 7` → Solid camel/tan vintage felt

### Background
`--bg color`
- color = 0-16 billiard ball color index for solid background

Example: `--bg 16` → White background

### Full Example
```bash
./target/release/ray-tracing 1 --custom \
  --ground 3 \
  --ball 0 0.2 1 1 3 \
  --bg 16
```
Creates: Navy blue felt + Circle green ball + White background

```bash
./target/release/ray-tracing 1 --custom \
  --ground --noise 5 \
  --ball 0 0.2 1 1 15 \
  --ball 1 0.2 1 1 12 \
 --ball 2 0.2 1 1 11 \
 --ball 3 0.2 1 1 10 \

  # Default noise ground (green)
./target/release/ray-tracing 1 --custom --ground

# Solid maroon ground (no noise)
./target/release/ray-tracing 1 --custom --ground 7

# Maroon noise ground (what you asked for!)
./target/release/ray-tracing 1 --custom --ground --noise 7

# Blue noise ground
./target/release/ray-tracing 1 --custom --ground --noise 1

# Any of the 17 billiard colors with noise texture
./target/release/ray-tracing 1 --custom --ground --noise 5  # Yellow noise