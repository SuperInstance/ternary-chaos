# Future Integration: ternary-chaos

## Current State
Provides iterated maps on {-1, 0, +1}, bifurcation detection, Lyapunov exponent estimation, strange attractor detection in ternary space, and period detection for nonlinear ternary dynamics.

## Integration Opportunities

### With ternary-cell
Cell tick cycles are iterated maps. When `TernaryCell::tick()` runs through its 6 phases (acquire → predict → update → surprise → vibe → gc), the aggregate grid state evolves as a high-dimensional iterated map. `ternary-chaos` can detect when the grid enters chaotic regimes — a sign that the cell population is destabilizing. Lyapunov exponent estimation on the grid state time series would provide early warning of cascade failures.

### With ternary-energy
Chaos theory directly connects to energy conservation. When a ternary system is near a bifurcation point, energy conservation tracking (`EnergyConservation`) will show increasing variance. The two crates together form a stability diagnostic: chaos detection for dynamics, energy tracking for conservation.

### With ternary-failure
The bifurcation detection maps directly to failure mode transitions. A `FailureMode::Critical` bifurcation in a room is when small perturbations cause large cascading effects — exactly what ternary-chaos detects via sensitivity-to-initial-conditions analysis.

## Potential in Mature Systems
In room-as-codespace, each room has its own dynamics. Chaos detection identifies rooms where agent behavior is unpredictable — candidates for tighter monitoring, simpler ensigns, or more conservative resource budgets. Strange attractors in room state suggest hidden stable operating patterns worth exploiting.

## Cross-Pollination Ideas
- Period detection on room state identifies cyclic patterns (e.g., daily load cycles) for predictive scheduling
- Lyapunov exponents as a room health metric — positive = unstable, negative = stable, zero = bifurcation imminent
- Chaos control: use ternary-chaos bifurcation parameters as control inputs to stabilize rooms

## Dependencies for Next Steps
- ternary-cell needs grid-level state history for Lyapunov computation
- Integration with ternary-failure for chaos-aware risk assessment
- ternary-room state logging at sufficient granularity for attractor detection
