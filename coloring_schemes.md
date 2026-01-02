# Escape-Time Fractal Coloring Schemes

## Continuous/Smooth Coloring

### Normalized Iteration Count
- Removes banding by using fractional iterations
- Formula: `n + 1 - log(log(|z|))/log(2)` for z² iteration
- Creates smooth gradients instead of discrete bands

### Renormalized Iteration
- Similar to above but uses: `n - log2(log(|z|)/log(bailout))`
- Adjusts for different escape radiuses

## Orbit-Based Coloring

### Orbit Traps
- Color based on how close the orbit gets to a geometric shape (point, line, circle, etc.)
- Minimum distance to trap determines color/brightness
- Creates entirely different patterns from the same fractal

### Average/Sum of Orbit
- Track the average position of all points in the orbit
- Or sum of distances from origin
- Reveals different structural information

### Angle/Argument Coloring
- Use the angle of the final z value: `atan2(Im(z), Re(z))`
- Creates rainbow-like color wheels
- Shows directional escape patterns

### Stripes/Rings
- Color based on `sin(angle * frequency)` or similar
- Creates concentric rings or radial stripes
- Adjustable frequency for different effects

## Distance Estimation

### Distance to Set Boundary
- Approximate distance to the fractal's edge
- Formula: `|z| * log(|z|) / |z'|` where z' is the derivative
- Useful for edge detection and 3D lighting effects

### Interior Distance
- For points that don't escape, estimate distance from interior
- Often uses periodicity detection or final z magnitude

## Multi-Channel Approaches

### RGB from Different Metrics
- R: iteration count
- G: orbit trap distance
- B: final z angle
- Combines multiple coloring methods

### Histogram Coloring
- Build histogram of iteration counts
- Redistribute colors to equalize frequency
- Prevents dominant colors from washing out detail

## Mathematical Transformations

### Log/Exponential Scaling
- Apply log or exp to iteration count before mapping to color
- Emphasizes different iteration ranges
- `log(n)`, `sqrt(n)`, `n^2`, etc.

### Trigonometric Palettes
- Use sine/cosine waves for RGB channels
- `R = sin(n * freq1 + phase1)`
- `G = sin(n * freq2 + phase2)`
- `B = sin(n * freq3 + phase3)`
- Creates complex, customizable color patterns

## Domain-Based Techniques

### Pickover Stalks
- Color based on real or imaginary part of final z
- `color = Im(z) / Re(z)` or similar
- Creates stalk-like structures

### Decomposition
- Color based on which quadrant/sector the orbit escapes through
- Binary decomposition: sign of Re(z) or Im(z)
- Creates symmetric patterns

## Advanced Methods

### Lyapunov Coloring
- Uses Lyapunov exponent: measures orbit stability
- More computationally intensive
- Reveals chaotic vs. stable regions

### Potential Function
- `μ = log(log(|z|)) / 2^n` for exterior
- Related to electric potential in physics
- Smooth, physically meaningful coloring

### Interior Coloring
- For non-escaping points:
  - Final z magnitude
  - Periodicity detection (color by period)
  - Lake coloring (constant for interior)

## Notes

The key is experimenting with combinations—many beautiful fractals use multiple techniques together (e.g., smooth iteration count with orbit traps and angle-based hue shifts).
