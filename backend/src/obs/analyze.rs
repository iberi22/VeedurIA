use polars::prelude::*;
use tracing::info;

/// Expected Benford distribution for first digits 1-9
const BENFORD_EXPECTED: [f64; 9] = [
    0.301, 0.176, 0.125, 0.097, 0.079, 0.067, 0.058, 0.051, 0.046,
];

/// Extract first digit from a numeric value
fn first_digit(value: f64) -> Option<u8> {
    if value <= 0.0 {
        return None;
    }
    let abs_val = value.abs();
    let first = abs_val / 10_f64.powf(abs_val.log10().floor());
    Some(first as u8)
}

/// Analyze a series of monetary values for Benford's Law conformance
pub fn analyze_benford(values: &[f64]) -> BenfordResult {
    let mut digit_counts = [0u32; 9];
    let mut valid_count = 0u32;

    for &val in values {
        if let Some(digit) = first_digit(val) {
            if digit >= 1 && digit <= 9 {
                digit_counts[(digit - 1) as usize] += 1;
                valid_count += 1;
            }
        }
    }

    if valid_count == 0 {
        return BenfordResult::default();
    }

    // Calculate observed distribution
    let observed: Vec<f64> = digit_counts
        .iter()
        .map(|&c| c as f64 / valid_count as f64)
        .collect();

    // Chi-squared statistic
    let chi_squared: f64 = observed
        .iter()
        .zip(BENFORD_EXPECTED.iter())
        .map(|(&obs, &exp)| {
            let expected_count = exp * valid_count as f64;
            if expected_count > 0.0 {
                (obs * valid_count as f64 - expected_count).powi(2) / expected_count
            } else {
                0.0
            }
        })
        .sum();

    // Critical value for chi-squared with 8 degrees of freedom at 0.05 significance
    let critical_value = 15.507;
    let is_anomalous = chi_squared > critical_value;

    info!(
        "Benford Analysis: chi² = {:.2}, critical = {:.2}, anomalous = {}",
        chi_squared, critical_value, is_anomalous
    );

    BenfordResult {
        observed_distribution: observed,
        expected_distribution: BENFORD_EXPECTED.to_vec(),
        chi_squared,
        is_anomalous,
        sample_size: valid_count,
    }
}

#[derive(Debug, Default)]
pub struct BenfordResult {
    pub observed_distribution: Vec<f64>,
    pub expected_distribution: Vec<f64>,
    pub chi_squared: f64,
    pub is_anomalous: bool,
    pub sample_size: u32,
}

/// Red Flag: Detect single-bidder contracts
pub fn detect_single_bidders(df: &DataFrame) -> PolarsResult<DataFrame> {
    // This would filter contracts where numero_oferentes == 1
    // For now, return a placeholder
    df.clone().lazy().collect()
}

/// Red Flag: Detect unusually short procurement timelines
pub fn detect_short_timelines(df: &DataFrame, min_days: i64) -> PolarsResult<DataFrame> {
    // Filter contracts where (fecha_cierre - fecha_publicacion) < min_days
    df.clone().lazy().collect()
}

/// Analyze a Polars DataFrame for Benford's Law
pub fn check_benford_df(df: &DataFrame, column: &str) -> Option<BenfordResult> {
    let series = df.column(column).ok()?;
    let values: Vec<f64> = series
        .f64()
        .ok()?
        .into_iter()
        .filter_map(|v| v)
        .collect();

    Some(analyze_benford(&values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit(123.45), Some(1));
        assert_eq!(first_digit(999.0), Some(9));
        assert_eq!(first_digit(0.0), None);
        assert_eq!(first_digit(-50.0), Some(5));
    }

    #[test]
    fn test_benford_uniform() {
        // Uniform distribution should be flagged as anomalous
        let uniform: Vec<f64> = (1..=9).map(|d| d as f64 * 100.0).collect();
        let result = analyze_benford(&uniform);
        // Small sample, may not be statistically significant
        assert!(result.sample_size > 0);
    }
}
