# ternary-chaos

**Chaos and nonlinear dynamics for ternary systems — iterated maps, bifurcation detection, Lyapunov exponents, and strange attractor analysis.**

## Background

Chaos theory studies deterministic systems that exhibit sensitive dependence on initial conditions — the "butterfly effect." Classical chaos is defined on continuous spaces (ℝ), where the Lyapunov exponent measures how quickly nearby trajectories diverge. But what happens when the state space is discrete, specifically the three-element set {−1, 0, +1}?

`ternary-chaos` explores this question by providing iterated maps on ternary states, bifurcation detection through parameter sweeps, Lyapunov exponent estimation, strange attractor detection, sensitivity analysis, and cycle enumeration. The crate is both a research tool for studying discrete dynamical systems and a practical library for analyzing chaotic behavior in the SuperInstance ecosystem.

A key finding (documented in the code) is that **true Lyapunov exponent estimation is fundamentally limited for discrete ternary maps**: the quantization through `Ternary::from_f64()` destroys infinitesimal perturbations immediately, making the classical perturbation-growth measurement inapplicable. This is a known limitation of discrete dynamical systems.

## How It Works

### Ternary State

`Ternary` has three variants: `Neg` (−1), `Zero` (0), `Pos` (+1). Conversion from continuous values uses thresholding: `< −0.5 → Neg`, `> 0.5 → Pos`, otherwise `Zero`.

### Iterated Maps

`TernaryMap` wraps an initial state, a parameter, and a rule function `fn(Ternary, f64) → Ternary`:

- **`default_rule(x, param)`** — Logistic-like: `param · x · (1 − |x|)`, then ternarize
- **`modulation_rule(x, param)`** — Additive: `x + param`, then ternarize
- **`xor_rule(x, param)`** — Flips sign when `param > 0.5`: Neg ↔ Pos, Zero unchanged

Iteration produces a trajectory stored in history. `detect_period(max_iterations)` searches for periodic orbits by checking for repeating subsequences.

### Bifurcation Detection

`detect_bifurcations()` sweeps a parameter across a range and records the orbit period at each value. Period changes indicate bifurcations — qualitative changes in system behavior. This mirrors the bifurcation diagrams seen in the logistic map.

### Lyapunov Exponent

`estimate_lyapunov()` tracks divergence between two trajectories starting from nearby initial conditions. **Known limitation**: because ternary rules immediately quantize through `from_f64()`, a perturbation of 0.001 is destroyed in the first iteration — the function cannot measure true chaos in discrete ternary dynamics. It is retained as a structural placeholder for future continuous-valued extensions.

### Strange Attractor Detection

`detect_strange_attractor()` runs a map for many iterations and analyzes:

- **Unique states visited** — a strange attractor visits multiple states
- **State transition diversity** — many unique (from, to) pairs
- **Non-periodicity** — no detectable period

A trajectory is classified as "strange" if it's non-periodic with ≥ 2 unique states and ≥ 2 unique transitions.

### Sensitivity Analysis

`sensitivity_to_initial_conditions()` measures the average divergence rate between all pairs of initial states over many iterations. `find_cycles()` exhaustively enumerates all minimal cycles up to a given length.

## Experimental Results

The test suite validates:

- **Ternary conversion** — `from_f64` correctly thresholds, `from_i8` maps exactly
- **Map iteration** — history grows correctly, `iterate_n` produces the right number of states
- **Period detection** — fixed points (period 1), period-2 orbits (XOR rule), and no-period cases
- **Bifurcation sweep** — produces the correct number of parameter points
- **Strange attractor** — XOR rule with param=1.0 is periodic (period 2), correctly classified as not strange
- **Sensitivity** — finite results for all parameter values
- **Cycle enumeration** — finds fixed points and period-2 cycles

### Key Observations

- The **XOR rule** produces a period-2 orbit: `Neg → Pos → Neg → ...`. This is the simplest non-trivial cycle.
- The **default rule** with param=0.5 maps everything to `Zero` (fixed point) because `0.5 · x · (1 − |x|)` always lands in `[−0.5, 0.5]`, which ternarizes to `Zero`.
- The **modulation rule** with param=1.0 shifts states upward: `Zero → Pos`, `Neg → Zero`, `Pos → Pos` (wraps through ternarization).

## Impact

`ternary-chaos` bridges complexity science and ternary computing. While true chaos requires continuous state spaces, the crate's analysis tools (period detection, bifurcation sweeps, cycle enumeration) are valuable for understanding the behavioral repertoire of any ternary dynamical system.

The sensitivity analysis has practical applications: in a ternary fleet, understanding which configurations are sensitive to perturbation (small changes cause large behavioral shifts) vs. robust (stable under perturbation) guides system design toward resilient configurations.

## Use Cases

1. **Fleet behavior analysis** — Model room state transitions as ternary maps. Use `detect_bifurcations()` to identify parameter ranges where the fleet transitions from stable (periodic) to unstable (aperiodic) behavior, informing configuration bounds.

2. **Rule design** — When designing new ternary rules for fleet decision-making, `find_cycles()` and `detect_period()` reveal the rule's behavioral repertoire: fixed points, cycles, or chaos. Rules with desirable cycles are preferred over chaotic ones.

3. **Sensitivity testing** — Before deploying a configuration change, `sensitivity_to_initial_conditions()` quantifies how sensitive the fleet is to perturbation under the new parameters, identifying configurations where small errors could cause large behavioral shifts.

4. **Strange attractor detection** — In long-running fleet simulations, `detect_strange_attractor()` identifies regimes where the system exhibits bounded non-periodic behavior — a hallmark of complex dynamics that may indicate emergent patterns.

5. **Educational research** — The crate provides a sandbox for exploring discrete chaos theory: students can define custom rules, sweep parameters, and visualize bifurcation diagrams, all within the ternary paradigm.

## Open Questions

- **Continuous-valued extensions:** The current `Ternary::from_f64()` quantization destroys the fine-grained information needed for Lyapunov exponent estimation. Should a future version support continuous-valued trajectories with optional ternary quantization at output?
- **Higher-dimensional maps:** The current maps are one-dimensional (single ternary state). Can the framework extend to coupled map lattices or multi-dimensional ternary dynamical systems?
- **Statistical measures:** Beyond period detection and Lyapunov estimation, should the crate include entropy measures, correlation dimension, or fractal dimension estimation for ternary trajectories?

## Connection to Oxide Stack

`ternary-chaos` provides the analytical layer for understanding dynamical behavior:

- **`ternary-fire`** — fire model histories can be analyzed for chaotic dynamics using Lyapunov estimation and bifurcation detection
- **`ternary-game-theory`** — game dynamics (best-response iteration) can be analyzed for periodic vs. chaotic behavior
- **`ternary-voting`** — consensus convergence can be studied through period detection and sensitivity analysis
- **`ternary-channel`** — message flow patterns in channels can be modeled as ternary dynamical systems

The crate's emphasis on quantized dynamics reflects the broader ecosystem's commitment to ternary representations, even when this imposes theoretical limitations (like Lyapunov estimation) that future work must address.
