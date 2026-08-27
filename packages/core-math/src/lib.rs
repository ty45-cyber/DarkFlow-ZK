use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketDepthConfig {
    pub buckets: Vec<f64>, // Normalized probabilities (0.0 to 1.0)
    pub baseline_intensity: f64, // mu for Hawkes process
}

/// Calculate Shannon Entropy H(X) = - Σ P(x) log2(P(x))
/// Evaluates the unpredictability of the dark order book depth.
pub fn compute_shannon_entropy(probabilities: &[f64]) -> f64 {
    probabilities
        .iter()
        .filter(|&&p| p > 0.0) // Ignore empty probability spaces to prevent NaN
        .fold(0.0, |acc, &p| acc - (p * p.log2()))
}

/// Simplified Hawkes Process scoring: λ(t) = μ + α Σ e^(-β(t - ti))
/// Detects bursts of order submissions (front-running/toxic flow attempts)
pub fn detect_toxic_flow_burst(
    arrival_times_ms: &[f64],
    current_time_ms: f64,
    decay_rate: f64, // beta
    jump_size: f64,  // alpha
) -> f64 {
    let mut excitation = 0.0;
    for &time in arrival_times_ms {
        if time < current_time_ms {
            excitation += jump_size * (-decay_rate * (current_time_ms - time)).exp();
        }
    }
    excitation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_max_uncertainty() {
        // Fair coin toss / completely distributed order book
        let uniform = vec![0.25, 0.25, 0.25, 0.25];
        let h = compute_shannon_entropy(&uniform);
        assert_eq!(h, 2.0); // 2 bits of entropy
    }
}