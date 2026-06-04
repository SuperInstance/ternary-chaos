# ternary-chaos

Chaos and nonlinear dynamics for ternary systems — iterated maps, Lyapunov exponents, bifurcation detection, strange attractor analysis, and cycle finding.

## Why This Exists

Chaos theory studies deterministic systems that are highly sensitive to initial conditions. Traditional chaos analysis works on continuous state spaces — the logistic map, Lorenz attractor, Hénon map. But when you discretize to three states {-1, 0, +1}, you get a different class of dynamical system: finite-state nonlinear maps with at most 3^3 = 27 possible transition rules.

Despite this simplicity, ternary maps exhibit surprisingly rich behavior — fixed points, limit cycles, period-doubling bifurcations, and non-periodic wandering across states. **ternary-chaos** provides tools to characterize this behavior: detect periods, estimate Lyapunov exponents, find bifurcation points, identify strange attractors, and enumerate all cycles.

## Core Concepts

| Type | Meaning |
|---|---|
| `Ternary` | State value: `Neg` (-1), `Zero` (0), `Pos` (+1) |
| `TernaryMap` | Iterated map with a custom rule, parameter, and history |
| `StrangeAttractorResult` | Analysis output: visits, transitions, periodicity, attractor detection |

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-chaos = "0.1"
```

```rust
use ternary_chaos::*;

fn main() {
    // Create a map with the built-in logistic-like rule
    let mut map = TernaryMap::new(Ternary::Pos, 1.0, TernaryMap::default_rule);

    // Iterate and observe
    let trajectory = map.iterate_n(20);
    println!("Trajectory: {:?}", trajectory);

    // Detect period
    let period = map.detect_period(100);
    println!("Period: {:?}", period);

    // Estimate Lyapunov exponent
    let lyapunov = estimate_lyapunov(TernaryMap::xor_rule, Ternary::Pos, 1.0, 100);
    println!("Lyapunov exponent: {:.4}", lyapunov);

    // Scan for bifurcations
    let bifurcations = detect_bifurcations(
        TernaryMap::default_rule,
        Ternary::Pos,
        (0.0, 2.0),
        50,
    );
    for (param, period) in &bifurcations {
        println!("param={:.2} period={:?}", param, period);
    }
}
```

## API Overview

### TernaryMap
- `new(initial, param, rule_fn)` — create with custom dynamics
- `iterate() → Ternary` — one step
- `iterate_n(n) → Vec<Ternary>` — batch iteration
- `detect_period(max_iters) → Option<usize>` — find orbit period
- `history() → &[Ternary]` — access full trajectory

### Built-in Rules
- `default_rule(x, param)` — logistic-like: `param * x * (1 - |x|)`
- `modulation_rule(x, param)` — additive: `x + param` then ternarize
- `xor_rule(x, param)` — sign flip when param > 0.5

### Analysis Functions
- `estimate_lyapunov(rule, initial, param, iterations) → f64` — perturbation-based Lyapunov exponent
- `detect_bifurcations(rule, initial, param_range, steps)` — sweep parameter, track period changes
- `detect_strange_attractor(rule, initial, param, iterations) → StrangeAttractorResult` — characterize attractor
- `sensitivity_to_initial_conditions(rule, param, states, iterations) → f64` — average divergence rate
- `find_cycles(rule, param, max_length) → Vec<Vec<Ternary>>` — enumerate all minimal cycles

## How It Works

**TernaryMap** wraps a rule function `fn(Ternary, f64) -> Ternary` that maps the current state and a parameter to the next state. The `default_rule` applies a logistic-like function `param * v * (1 - |v|)` and ternarizes the result. With parameter = 0 everything collapses to Zero; as the parameter increases, richer dynamics emerge.

**Period detection** iterates the map and checks if the trajectory tail repeats with period *p* by comparing the last *2p* values. **Lyapunov exponent** estimation tracks two nearby initial conditions, measures how quickly they diverge (or converge), and averages the log divergence rate over many iterations. Positive Lyapunov exponents indicate chaos.

**Bifurcation detection** sweeps the parameter across a range, running period detection at each step. Period changes (e.g., period-1 → period-2 → non-periodic) mark bifurcation points. **Strange attractor detection** runs a long trajectory, counts unique states and transitions, and flags non-periodic behavior that visits a limited subset of states as a potential strange attractor.

**Cycle finding** exhaustively tests all starting states, iterates for each candidate cycle length, verifies the cycle closes, and eliminates duplicates and non-minimal cycles (shorter cycles that repeat).

## Use Cases

- **Cryptographic sequence generation** — analyze ternary maps for non-periodic, high-Lyapunov behavior suitable for pseudo-random ternary sequence generation
- **Biological modeling** — model gene regulatory networks with ternary states (down/neutral/up) and study their dynamical properties
- **Control system verification** — check ternary control maps for unwanted chaotic regimes or unexpected limit cycles across parameter ranges

## Ecosystem

Part of the **SuperInstance** ternary computing ecosystem:

- [`ternary`](https://crates.io/crates/ternary) — core trit types and balanced ternary arithmetic
- [`ternary-chaos`](https://crates.io/crates/ternary-chaos) — this crate
- [`ternary-circuit`](https://crates.io/crates/ternary-circuit) — ternary logic gates and circuits
- [`ternary-control`](https://crates.io/crates/ternary-control) — ternary control theory
- [`ternary-fuzzy`](https://crates.io/crates/ternary-fuzzy) — fuzzy logic with ternary membership

## License

MIT
