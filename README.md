# Rust Ray TracerFirst ball: --ball 0 0.2 1 5 0



This is a compact educational ray tracer in one source file that outputs an ASCII PPM image to stdout.

This is a compact educational ray tracer in one source file that outputs an ASCII PPM image to stdout.

## Build & run

## Build & run

```bash0 = number_type (0 = Circle, 1 = Line, like ball #1 in pool)

cargo build --release

# produce an image (scene 3, 800x600, recursion depth 3)

./target/release/ray-tracing > output.ppm```bash

```cargo build --release

# produce an image (scene 3, 800x600, recursion depth 3)

## Scenes

1 - single sphere## Scenes

2 - plane + cube (lower brightness)3 - all objects (sphere, cube, cylinder, plane)

3 - all objects (sphere, cube, cylinder, plane)4 - same as 3 but different camera position

4 - same as 3 but different camera position`--ball 0 0.2 1 yellow 1` → Yellow ball at (0, 0.2, 1) with Line pattern

`--ball 0 0.2 1 blue2 0` → Blue striped ball with Circle pattern

## Custom Scene Arguments`--ball 0 0.2 1 5 1` → Yellow ball (index 5) with Line pattern

`--ball 0 0.2 1 purple2` → Purple striped ball with random pattern (0 or 1)

### BallsFirst ball: --ball 0 0.2 1 0 5

`--ball x y z color [number_type]`

0 = x coordinate

- x, y, z = position coordinates0.2 = y coordinate

- color (REQUIRED): ball index 0-16 OR color name1 = z coordinate

- number_type (optional): 0 = Circle, 1 = Line (random if omitted)

### Balls

**Color Names:**`--ball x y z [number_type] [color]`

- Solid: `black`, `blue`, `purple`, `green`, `orange`, `yellow`, `red`, `maroon`, `white`

- Striped: `yellow2`, `blue2`, `red2`, `purple2`, `orange2`, `green2`, `maroon2`, `black2`- x, y, z = position coordinates

- number_type (optional): 0 = Circle, 1 = Line (random if omitted)

Examples: - color (optional): ball index 0-16 OR color name (random if omitted)

- `--ball 0 0.2 1 yellow 1` → Yellow ball at (0, 0.2, 1) with Line pattern

- `--ball 0 0.2 1 blue2 0` → Blue striped ball with Circle pattern**Color Names:**

- `--ball 0 0.2 1 5 1` → Yellow ball (index 5) with Line pattern- Solid: `black`, `blue`, `purple`, `green`, `orange`, `yellow`, `red`, `maroon`, `white`

- `--ball 0 0.2 1 purple2` → Purple striped ball with random pattern (0 or 1)

Examples: 

<!-- The old --plane flag has been deprecated in favor of the simpler --ground convenience flag. -->

### Ground (convenience)

### Ground (convenience)`--ground [--noise] [color]`

`--ground [--noise] [color]`- By default (no arguments), uses Perlin noise with tournament green felt

- Creates a horizontal ground plane at the origin (0,0,0) with normal (0,1,0)- `--noise` flag enables Perlin noise texture

- By default (no arguments), uses Perlin noise with tournament green felt- `color` (optional) = 0-9 billiard table felt color index:

- `--noise` flag enables Perlin noise texture  - 0: Tournament Green (default)

- `color` (optional) = 0-5 billiard table felt color index:  - 1: Dark Green (traditional)

  - 0: Tournament Green (default)  - 2: English Green

  - 1: Electric Blue  - 3: Navy Blue

  - 2: Burgundy/Wine Red  --ball 0 0.2 1 3 1 \

  - 3: PAF green  - 5: Burgundy/Wine Red

  - 4: PAF dark  - 6: Dark Red

  - 5: PAF dark smoothCreates: Navy blue felt + Purple ball with Line pattern + White background

- Without `--noise` and with a color index, creates a solid colored felt  - 8: Charcoal Grey

  - 9: Purple (modern)

Examples:- Without `--noise` and with a color index, creates a solid colored felt

- `--ground` → Perlin noise ground with default tournament green

- `--ground 3` → Solid PAF green felt (no noise)  --ball 0 0.2 1 15 1 \

- `--ground --noise 3` → PAF green felt with Perlin noise texture  --ball 1 0.2 1 12 1 \

- `--ground --noise 2` → Burgundy felt with Perlin noise texture  --ball 2 0.2 1 11 1 \

  --ball 3 0.2 1 10 1 \

### Background- `--ground --noise 5` → Burgundy felt with Perlin noise texture

`--bg color`- `--ground 7` → Solid camel/tan vintage felt

- color = ball color index 0-16 or color name for solid background

### Background

Examples:`--bg color`

- `--bg 16` → White background- color = 0-16 billiard ball color index for solid background

- `--bg white` → White background

Example: `--bg 16` → White background

### Full Example

```bash### Full Example

./target/release/ray-tracing 1 --custom \```bash

  --ground 3 \./target/release/ray-tracing 1 --custom \

  --ball 0 0.2 1 purple 1 \  --ground 3 \

  --bg 16  --ball 0 0.2 1 1 3 \

```  --bg 16

Creates: PAF green felt + Purple ball with Line pattern + White background```

Creates: Navy blue felt + Circle green ball + White background

```bash

./target/release/ray-tracing 1 --custom \```bash

  --ground --noise 2 \./target/release/ray-tracing 1 --custom \

  --ball 0 0.2 1 maroon2 1 \  --ground --noise 5 \

  --ball 1 0.2 1 red2 1 \  --ball 0 0.2 1 1 15 \

  --ball 2 0.2 1 yellow2 1 \  --ball 1 0.2 1 1 12 \

  --ball 3 0.2 1 blue2 1 --ball 2 0.2 1 1 11 \

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

## Custom Scene Arguments

### Balls
`--ball x y z color [number_type] [yaw pitch]`

- x, y, z = position coordinates
- color (REQUIRED): ball index 0-16 OR color name
- number_type (optional): 0 = Circle, 1 = Line (random if omitted)
- orientation (optional 2 floats): yaw = rotation around vertical axis, pitch = rotation around horizontal axis; if omitted a random orientation is chosen.

**Color Names:**
- Solid: `black`, `blue`, `purple`, `green`, `orange`, `yellow`, `red`, `maroon`, `white`
- Striped: `yellow2`, `blue2`, `red2`, `purple2`, `orange2`, `green2`, `maroon2`, `black2`

Examples:
- `--ball 0 0.2 1 yellow 1` → Yellow ball with Line pattern
- `--ball 0 0.2 1 blue2 0` → Blue striped ball with Circle pattern
- `--ball 0 0.2 1 5 1` → Yellow ball (index 5) with Line pattern
- `--ball 0 0.2 1 purple2` → Purple striped ball (random orientation)
- `--ball 1 0.2 2 maroon2 0 30 15` → Maroon striped ball with stripe orientation yaw 30° pitch 15°
- `--ball 2 0.2 3 blue 1 0 45` → Solid blue ball with numbered spot centered on yaw 0° pitch 45°

### Table (convenience)
`--table [--texture <felt name>]`
- Creates a horizontal ground plane at the origin (0,0,0) with normal (0,1,0)
- By default (no arguments), creates solid Tournament Green felt
- `--texture <felt name>` enables Perlin noise felt with the named color
- Accepted felt names (case-insensitive): Tournament Green, Electric Blue, Burgundy (Wine Red), PAF green, PAF dark, PAF dark smooth

Examples:
- `--table` → Solid Tournament Green felt
- `--table --texture PAF green` → Perlin noise felt with PAF green
- `--table --texture Electric Blue` → Perlin noise felt with Electric Blue

### Background
`--bg color`
- color = ball color index 0-16 or color name for solid background

Examples:
- `--bg 16` → White background
- `--bg white` → White background

### Full Example
```bash
./target/release/ray-tracing 1 --custom \
  --table --texture PAF green \
  --ball 0 0.2 1 purple 1 0 1 0 \
  --bg 16
```
Creates: PAF green felt + Purple ball (Line) oriented with axis +Y + White background

```bash
./target/release/ray-tracing 1 --custom \
  --table --texture Burgundy \
  --ball 0 0.2 1 maroon2 1 0 0 1 \
  --ball 1 0.2 1 red2 1 \
  --ball 2 0.2 1 yellow2 1 \
  --ball 3 0.2 1 blue2 1
```

More examples:
```bash
# Solid Tournament Green table
./target/release/ray-tracing 1 --custom --table

# Electric Blue felt texture
./target/release/ray-tracing 1 --custom --table --texture Electric Blue

# Burgundy felt texture
./target/release/ray-tracing 1 --custom --table --texture Burgundy
```
Yaw Range: 0° to 360° (or -180° to 180°)
Yaw can be any value because it's a full rotation around the vertical axis
0° to 360°: Full circle (0° = 360°)
Negative values work too: -90° is the same as 270°
You can use values like 0°, 45°, 90°, 180°, 270°, 360°, or even 450° (equivalent to 90°)
Pitch Range: -90° to 90°
Pitch is limited to -90° to 90° because:
0° = horizontal (parallel to table)
90° = straight up (perpendicular to table, pointing top)
-90° = straight down (perpendicular to table, pointing bottom)
Values beyond ±90° don't make physical sense (you'd be going "past vertical")

 `target/release/ray-tracing 1 --custom --table --texture PAF darker --ball 3 0.2 2 purple2 1 180 90 --ball 3 0.2 1 purple 1 180 90 --ball 2.5 0.2 2 orange2 1 180 90 --ball 2.5 0.2 1 orange 1 180 90 --ball 2 0.2 2 red2 1 180 90 --ball 2 0.2 1 red 1 180 90 --ball 1.5 0.2 2 yellow2 1 180 90 --ball 1.5 0.2 1 yellow 1 180 90 --ball 1 0.2 2 green2 1 180 90 --ball 1 0.2 1 green 1 180 90 --ball 0.5 0.2 2 blue2 1 180 90 --ball 0.5 0.2 1 blue 1 180 90`