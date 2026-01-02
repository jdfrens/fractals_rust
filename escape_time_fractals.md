# Escape-Time Fractal Algorithms

## Classic Variations

### Burning Ship
- Uses `z = (|Re(z)| + i|Im(z)|)² + c` instead of `z²+c`
- Creates ship-like structures with intricate detail
- Known for its unusual asymmetry

### Tricorn (Mandelbar)
- Uses the conjugate: `z = z̄² + c`
- Produces a heart-shaped main body with unique filaments
- Anti-holomorphic, leading to different mathematical properties

### Phoenix
- Iteration: `z_new = z² + Re(c) + Im(c)·z_prev`
- Requires tracking the previous iteration value
- Creates flame-like or phoenix-wing patterns

## Higher-Order Polynomials

### Multibrot Sets
- General form: `z = z^n + c` where n > 2
- n=3 (cubic) creates triangular symmetry
- n=4 (quartic) creates square symmetry
- Higher powers create increasingly complex symmetry patterns

### Nova Fractals
- Combines Newton's method with escape-time: `z = z - (z^n - 1)/(n·z^(n-1)) + c`
- Creates colorful, flower-like patterns with distinct convergence basins
- Blends root-finding with traditional escape behavior

## Alternative Formulas

### Magnet Fractals
- Based on magnetic renormalization transformations from physics
- Type 1: `z = ((z² + c - 1)/(2z + c - 2))²`
- Type 2: More complex rational function
- Creates smooth, organic-looking structures

### Lambda (λ) Fractals
- `z = λ·z·(1 - z)`
- Related to logistic map and bifurcation diagrams
- Different dynamic behavior from polynomial fractals

### Exponential/Transcendental
- `z = e^z + c` or `z = sin(z) + c`
- Infinite periodic structures
- Very different visual character from polynomial fractals

## Multi-dimensional Approaches

### Quaternion/Hypercomplex Fractals
- Extend to 4D using quaternions
- 3D slices reveal volumetric fractal structures
- Mathematically rich but computationally intensive

### Biomorphs
- Check both `|Re(z)|` and `|Im(z)|` against escape radius separately
- Creates biological or insect-like forms
- Introduced by Clifford Pickover
