#![forbid(unsafe_code)]

//! Chaos and nonlinear dynamics for ternary systems.
//!
//! Provides iterated maps on {-1, 0, +1}, bifurcation detection,
//! Lyapunov exponent estimation, strange attractor detection in ternary space,
//! sensitivity to initial conditions analysis, and period detection.

use std::collections::HashMap;

/// A ternary state value: -1, 0, or +1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    Neg,
    Zero,
    Pos,
}

impl Ternary {
    pub fn value(&self) -> i8 {
        match self {
            Ternary::Neg => -1,
            Ternary::Zero => 0,
            Ternary::Pos => 1,
        }
    }

    pub fn from_i8(v: i8) -> Option<Self> {
        match v {
            -1 => Some(Ternary::Neg),
            0 => Some(Ternary::Zero),
            1 => Some(Ternary::Pos),
            _ => None,
        }
    }

    pub fn from_f64(v: f64) -> Self {
        if v < -0.5 { Ternary::Neg }
        else if v > 0.5 { Ternary::Pos }
        else { Ternary::Zero }
    }

    /// Apply a nonlinear map: multiply by parameter, then ternarize.
    pub fn nonlinear_map(self, param: f64) -> Self {
        let v = self.value() as f64 * param;
        Ternary::from_f64(v)
    }
}

/// An iterated map on ternary states.
#[derive(Clone, Debug)]
pub struct TernaryMap {
    /// Rule table: maps (current_state, parameter_bucket) -> next_state
    /// We represent it as a function via a closure-like approach.
    rule: fn(Ternary, f64) -> Ternary,
    pub state: Ternary,
    pub param: f64,
    history: Vec<Ternary>,
}

impl TernaryMap {
    pub fn new(initial: Ternary, param: f64, rule: fn(Ternary, f64) -> Ternary) -> Self {
        TernaryMap {
            rule,
            state: initial,
            param,
            history: vec![initial],
        }
    }

    /// Default ternary logistic-like map.
    pub fn default_rule(x: Ternary, param: f64) -> Ternary {
        let v = x.value() as f64;
        let mapped = param * v * (1.0 - v.abs());
        Ternary::from_f64(mapped)
    }

    /// Modulation map: adds param and ternarizes.
    pub fn modulation_rule(x: Ternary, param: f64) -> Ternary {
        let v = x.value() as f64 + param;
        Ternary::from_f64(v)
    }

    /// XOR-like map: flips sign based on param.
    pub fn xor_rule(x: Ternary, param: f64) -> Ternary {
        if param > 0.5 {
            match x {
                Ternary::Neg => Ternary::Pos,
                Ternary::Pos => Ternary::Neg,
                Ternary::Zero => Ternary::Zero,
            }
        } else { x }
    }

    pub fn iterate(&mut self) -> Ternary {
        self.state = (self.rule)(self.state, self.param);
        self.history.push(self.state);
        self.state
    }

    pub fn iterate_n(&mut self, n: usize) -> Vec<Ternary> {
        (0..n).map(|_| self.iterate()).collect()
    }

    pub fn history(&self) -> &[Ternary] {
        &self.history
    }

    /// Detect the period of the current orbit.
    pub fn detect_period(&mut self, max_iterations: usize) -> Option<usize> {
        let initial = self.state;
        let mut trajectory = vec![initial];
        for _ in 0..max_iterations {
            self.state = (self.rule)(self.state, self.param);
            trajectory.push(self.state);
            let len = trajectory.len();
            for period in 1..=len / 2 {
                let mut is_periodic = true;
                for i in 0..period {
                    if len < 2 * period { break; }
                    if trajectory[len - 1 - i] != trajectory[len - 1 - i - period] {
                        is_periodic = false;
                        break;
                    }
                }
                if is_periodic && len >= 2 * period {
                    return Some(period);
                }
            }
        }
        None
    }
}

/// Estimate the Lyapunov exponent for a ternary map.
/// Uses perturbation sensitivity: track how small differences grow.
///
/// **Known limitation:** Because ternary rules immediately quantize through
/// `Ternary::from_f64`, a perturbation of 0.001 is destroyed in the first
/// iteration (e.g. `from_f64(1.0)` and `from_f64(1.001)` both yield `Pos`).
/// This means the returned exponent is always effectively zero for any rule
/// that ternarizes — the function cannot measure true chaos in discrete
/// ternary dynamics. It is retained as a structural placeholder for future
/// continuous-valued extensions.
pub fn estimate_lyapunov<F>(rule: F, initial: Ternary, param: f64, iterations: usize) -> f64
where
    F: Fn(Ternary, f64) -> Ternary,
{
    let perturbation = 0.001;
    let mut state1 = initial.value() as f64;
    let mut state2 = initial.value() as f64 + perturbation;
    let mut sum = 0.0;
    let mut count = 0;

    for _ in 0..iterations {
        let t1 = Ternary::from_f64(state1);
        let t2 = Ternary::from_f64(state2);
        let next1 = rule(t1, param);
        let next2 = rule(t2, param);
        let diff = (next2.value() - next1.value()) as f64;
        let denom = perturbation;
        if denom.abs() > 1e-15 {
            sum += (diff.abs() / denom).max(1e-15).ln();
            count += 1;
        }
        state1 = next1.value() as f64;
        state2 = next2.value() as f64;
    }

    if count == 0 { 0.0 } else { sum / count as f64 }
}

/// Detect bifurcations by sweeping a parameter and checking for period changes.
pub fn detect_bifurcations(rule: fn(Ternary, f64) -> Ternary, initial: Ternary, param_range: (f64, f64), steps: usize) -> Vec<(f64, Option<usize>)>
{
    let mut results = Vec::new();
    let (p_min, p_max) = param_range;
    let step_size = (p_max - p_min) / steps as f64;

    for i in 0..=steps {
        let param = p_min + i as f64 * step_size;
        let mut map = TernaryMap::new(initial, param, rule);
        let period = map.detect_period(100);
        results.push((param, period));
    }
    results
}

/// Detect strange attractors in ternary space.
/// A strange attractor is identified by non-periodic behavior that visits
/// a limited subset of ternary states repeatedly.
pub fn detect_strange_attractor(rule: fn(Ternary, f64) -> Ternary, initial: Ternary, param: f64, iterations: usize) -> StrangeAttractorResult
{
    let mut state = initial;
    let mut visits: HashMap<Ternary, usize> = HashMap::new();
    let mut trajectory = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        state = rule(state, param);
        *visits.entry(state).or_insert(0) += 1;
        trajectory.push(state);
    }

    // Check if it's periodic
    let mut map = TernaryMap::new(initial, param, rule);
    let period = map.detect_period(iterations.min(200));

    // Count state transitions (unique pairs)
    let mut transitions: HashMap<(Ternary, Ternary), usize> = HashMap::new();
    for i in 1..trajectory.len() {
        *transitions.entry((trajectory[i - 1], trajectory[i])).or_insert(0) += 1;
    }

    let unique_states = visits.len();
    let unique_transitions = transitions.len();
    let is_strange = period.is_none() && unique_states >= 2 && unique_transitions >= 2;

    StrangeAttractorResult {
        is_strange,
        visits,
        transitions: unique_transitions,
        period,
        unique_states,
    }
}

/// Result of strange attractor detection.
#[derive(Debug)]
pub struct StrangeAttractorResult {
    pub is_strange: bool,
    pub visits: HashMap<Ternary, usize>,
    pub transitions: usize,
    pub period: Option<usize>,
    pub unique_states: usize,
}

/// Measure sensitivity to initial conditions.
/// Returns the average divergence rate between two nearby initial states.
pub fn sensitivity_to_initial_conditions<F>(
    rule: F,
    param: f64,
    states: &[Ternary],
    iterations: usize,
) -> f64
where
    F: Fn(Ternary, f64) -> Ternary,
{
    if states.len() < 2 { return 0.0; }

    let mut divergences = Vec::new();
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            let mut s1 = states[i];
            let mut s2 = states[j];
            let mut diff_count = 0;

            for _ in 0..iterations {
                s1 = rule(s1, param);
                s2 = rule(s2, param);
                if s1 != s2 {
                    diff_count += 1;
                }
            }
            divergences.push(diff_count as f64 / iterations as f64);
        }
    }

    if divergences.is_empty() { 0.0 } else { divergences.iter().sum::<f64>() / divergences.len() as f64 }
}

/// Find all cycles of a given length for a ternary map.
pub fn find_cycles<F>(rule: F, param: f64, max_length: usize) -> Vec<Vec<Ternary>>
where
    F: Fn(Ternary, f64) -> Ternary + Copy,
{
    let all_states = [Ternary::Neg, Ternary::Zero, Ternary::Pos];
    let mut cycles = Vec::new();

    for length in 1..=max_length {
        // Try all possible starting states
        for &start in &all_states {
            let mut cycle = vec![start];
            let mut state = start;
            let mut valid = true;

            for _ in 1..length {
                state = rule(state, param);
                cycle.push(state);
            }

            // Verify it cycles back
            let next = rule(state, param);
            if next == start && cycle.len() == length {
                // Check it's not a shorter cycle repeated
                let mut is_minimal = true;
                for div in 1..length {
                    if length % div == 0 {
                        let mut repeats = true;
                        for k in 0..length {
                            if cycle[k] != cycle[k % div] {
                                repeats = false;
                                break;
                            }
                        }
                        if repeats { is_minimal = false; break; }
                    }
                }
                if is_minimal {
                    cycles.push(cycle);
                }
            }
        }
    }

    // Deduplicate
    let mut seen: HashMap<Vec<Ternary>, bool> = HashMap::new();
    cycles.retain(|c| seen.insert(c.clone(), true).is_none());
    cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_values() {
        assert_eq!(Ternary::Neg.value(), -1);
        assert_eq!(Ternary::Zero.value(), 0);
        assert_eq!(Ternary::Pos.value(), 1);
    }

    #[test]
    fn test_ternary_from_f64() {
        assert_eq!(Ternary::from_f64(-1.0), Ternary::Neg);
        assert_eq!(Ternary::from_f64(0.0), Ternary::Zero);
        assert_eq!(Ternary::from_f64(1.0), Ternary::Pos);
        assert_eq!(Ternary::from_f64(0.3), Ternary::Zero);
    }

    #[test]
    fn test_ternary_nonlinear_map() {
        let mapped = Ternary::Pos.nonlinear_map(0.5);
        assert_eq!(mapped, Ternary::Zero); // 1 * 0.5 * (1 - 1) = 0
    }

    #[test]
    fn test_ternary_map_iteration() {
        let mut map = TernaryMap::new(Ternary::Pos, 0.5, TernaryMap::default_rule);
        let _result = map.iterate();
        assert!(map.history().len() == 2);
    }

    #[test]
    fn test_ternary_map_iterate_n() {
        let mut map = TernaryMap::new(Ternary::Pos, 0.5, TernaryMap::default_rule);
        let results = map.iterate_n(10);
        assert_eq!(results.len(), 10);
        assert_eq!(map.history().len(), 11);
    }

    #[test]
    fn test_period_detection_fixed_point() {
        let mut map = TernaryMap::new(Ternary::Zero, 0.5, TernaryMap::default_rule);
        let period = map.detect_period(50);
        assert_eq!(period, Some(1)); // Zero maps to Zero (fixed point)
    }

    #[test]
    fn test_period_detection_xor() {
        // XOR rule with param > 0.5 oscillates: Neg <-> Pos
        let mut map = TernaryMap::new(Ternary::Neg, 1.0, TernaryMap::xor_rule);
        let period = map.detect_period(50);
        assert_eq!(period, Some(2)); // Period 2: Neg -> Pos -> Neg
    }

    #[test]
    fn test_modulation_rule() {
        let mut map = TernaryMap::new(Ternary::Zero, 1.0, TernaryMap::modulation_rule);
        let state = map.iterate();
        assert_eq!(state, Ternary::Pos); // 0 + 1 = 1 -> Pos
    }

    #[test]
    fn test_xor_rule() {
        assert_eq!(TernaryMap::xor_rule(Ternary::Neg, 1.0), Ternary::Pos);
        assert_eq!(TernaryMap::xor_rule(Ternary::Pos, 1.0), Ternary::Neg);
        assert_eq!(TernaryMap::xor_rule(Ternary::Zero, 1.0), Ternary::Zero);
    }

    #[test]
    fn test_xor_rule_no_flip() {
        assert_eq!(TernaryMap::xor_rule(Ternary::Pos, 0.0), Ternary::Pos);
        assert_eq!(TernaryMap::xor_rule(Ternary::Neg, 0.0), Ternary::Neg);
    }

    #[test]
    fn test_lyapunov_estimation() {
        let lyap = estimate_lyapunov(TernaryMap::xor_rule, Ternary::Pos, 1.0, 100);
        // Should be finite
        assert!(lyap.is_finite());
    }

    #[test]
    fn test_bifurcation_detection() {
        let results = detect_bifurcations(TernaryMap::default_rule, Ternary::Pos, (0.0, 2.0), 20);
        assert_eq!(results.len(), 21);
        // All should have a param value
        for (param, _) in &results {
            assert!(*param >= 0.0 && *param <= 2.0);
        }
    }

    #[test]
    fn test_strange_attractor_detection() {
        let result = detect_strange_attractor(TernaryMap::xor_rule, Ternary::Pos, 1.0, 100);
        // XOR with param 1.0 is periodic (period 2), so not strange
        assert!(!result.is_strange);
        assert!(result.unique_states >= 1);
    }

    #[test]
    fn test_strange_attractor_visits() {
        let result = detect_strange_attractor(TernaryMap::default_rule, Ternary::Pos, 0.5, 100);
        assert!(result.visits.len() >= 1);
    }

    #[test]
    fn test_sensitivity_zero_param() {
        let sens = sensitivity_to_initial_conditions(
            TernaryMap::default_rule,
            0.0,
            &[Ternary::Pos, Ternary::Neg],
            50,
        );
        // Everything maps to Zero with param 0, so low sensitivity
        assert!(sens.is_finite());
    }

    #[test]
    fn test_sensitivity_xor() {
        let sens = sensitivity_to_initial_conditions(
            TernaryMap::xor_rule,
            1.0,
            &[Ternary::Pos, Ternary::Neg, Ternary::Zero],
            50,
        );
        assert!(sens >= 0.0);
    }

    #[test]
    fn test_find_cycles_identity() {
        // With param 0, default_rule maps everything to Zero eventually
        let cycles = find_cycles(TernaryMap::default_rule, 0.0, 3);
        // Zero -> Zero is a cycle of length 1
        assert!(cycles.iter().any(|c| c.len() == 1 && c[0] == Ternary::Zero));
    }

    #[test]
    fn test_find_cycles_xor() {
        let cycles = find_cycles(TernaryMap::xor_rule, 1.0, 3);
        // Neg -> Pos -> Neg is a cycle of length 2
        assert!(cycles.iter().any(|c| c.len() == 2));
    }

    #[test]
    fn test_find_cycles_zero_fixed_point() {
        let cycles = find_cycles(|x, _| x, 0.0, 3);
        // Identity: all fixed points
        assert!(cycles.len() >= 3);
    }

    #[test]
    fn test_ternary_map_set_param() {
        let mut map = TernaryMap::new(Ternary::Pos, 0.5, TernaryMap::default_rule);
        map.param = 2.0;
        assert_eq!(map.param, 2.0);
    }

    #[test]
    fn test_empty_sensitivity() {
        let sens = sensitivity_to_initial_conditions(
            TernaryMap::default_rule, 0.5, &[], 10,
        );
        assert_eq!(sens, 0.0);
    }
}
