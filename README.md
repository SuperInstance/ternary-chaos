# Ternary Chaos

**Ternary Chaos** provides chaos theory and nonlinear dynamics tools for ternary systems — iterated maps on {-1, 0, +1}, bifurcation detection, Lyapunov exponent estimation, strange attractor detection, sensitivity-to-initial-conditions analysis, and period detection.

## Why It Matters

Ternary systems can exhibit chaotic behavior — small perturbations in initial conditions lead to dramatically different trajectories. Understanding chaos in ternary agent populations is critical for predicting fleet stability: will a small change in one agent's strategy cascade into fleet-wide reorganization, or will it damp out? Ternary Chaos provides the mathematical tools to answer this, measuring sensitivity (Lyapunov exponents), detecting periodic and chaotic regimes (bifurcation diagrams), and identifying attractor structures.

## How It Works

### Iterated Maps on Ternary States

```
TernaryMap {
    rule: fn(Ternary, f64) -> Ternary,
    state: Ternary,
    param: f64,
}

step(): state = rule(state, param)
```

The nonlinear map applies a parameter-dependent transform then re-ternarizes:

```
v = state.value() × param
new_state = ternarize(v)  // < -0.5 → Neg, > 0.5 → Pos, else Zero
```

### Lyapunov Exponent

Measures sensitivity to initial conditions:

```
λ = lim(t→∞) (1/t) · Σ ln|f'(x_i)|

λ > 0: chaotic (exponential divergence)
λ = 0: neutral/marginal
λ < 0: stable (exponential convergence)
```

Estimation: iterate two nearby initial conditions, measure separation rate. Cost: **O(N)** for N iterations.

### Bifurcation Detection

As parameter μ varies, the system may transition from fixed point → periodic → chaotic:

```
for μ in μ_min..μ_max:
    iterate map 1000 times (transient)
    record next 100 states (attractor)
    classify: fixed point (1 unique), periodic (k unique), chaotic (>k unique)
```

Full bifurcation diagram: **O(M · N)** for M parameter values, N iterations each.

### Period Detection

Floyd's cycle detection on the ternary state sequence:

```
Phase 1: tortoise = f(x), hare = f(f(x)) — advance until equal
Phase 2: find cycle length λ by counting steps until repeat
```

Cost: **O(μ + λ)** time, **O(1)** space.

### Strange Attractor Detection

A strange attractor has fractal dimension and aperiodic dynamics:

```
- Correlation dimension (Grassberger-Procaccia): O(N²) pairwise distances
- Positive Lyapunov exponent: O(N) iteration
- Recurrence plot structure: O(N²) threshold comparison
```

### Sensitivity Analysis

```
sensitivity(perturbation):
    original = simulate(N steps)
    perturbed = simulate(N steps, perturbed_initial)
    divergence = ||original - perturbed||
    return divergence trend (growing = sensitive, stable = robust)
```

Cost: **O(N)** per comparison.

## Quick Start

```rust
use ternary_chaos::{TernaryMap, Ternary};

let mut map = TernaryMap::new(|s, p| {
    Ternary::from_f64(s.value() as f64 * p)
}, Ternary::Zero, 1.5);

for _ in 0..1000 { map.step(); }
let history = map.history();
println!("Period: {:?}", map.detect_period());
```

## API

| Type | Description |
|------|-------------|
| `TernaryMap` | Iterated map with rule, state, parameter, history |
| `Ternary` | Neg (-1), Zero (0), Pos (+1) |
| `lyapunov_exponent()` | Sensitivity to initial conditions |
| `detect_period()` | Floyd's cycle detection |
| `bifurcation_diagram()` | Parameter sweep with regime classification |

## Architecture Notes

Ternary Chaos provides the nonlinear dynamics analysis layer for fleet stability prediction in SuperInstance. In γ + η = C, chaotic regimes indicate that the conservation law itself may be unstable — small perturbations to γ or η can cause disproportionate changes to C. The Lyapunov exponent serves as an early warning: positive λ means the system is on the edge of chaos, and intervention (increasing η damping) may be needed.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for nonlinear dynamics architecture.

## References

1. Strogatz, S. H. (2018). *Nonlinear Dynamics and Chaos*, 2nd ed. Westview Press.
2. Lorenz, E. N. (1963). "Deterministic Nonperiodic Flow." *Journal of the Atmospheric Sciences*, 20(2), 130–141.
3. Grassberger, P. & Procaccia, I. (1983). "Measuring the Strangeness of Strange Attractors." *Physica D*, 9(1-2), 189–208.

## License

MIT
