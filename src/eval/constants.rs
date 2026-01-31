//! Mathematical constant registry.
//!
//! Provides a static registry of mathematical constants used for evaluation
//! context initialization and constant recognition.

/// A mathematical constant with its name and value.
pub static MATH_CONSTANTS: &[(&str, f64)] = &[
    ("pi", std::f64::consts::PI),
    ("e", std::f64::consts::E),
    ("tau", std::f64::consts::TAU),
    ("phi", 1.618_033_988_749_895),
    ("sqrt2", std::f64::consts::SQRT_2),
    ("sqrt3", 1.732_050_807_568_877_2),
    ("ln2", std::f64::consts::LN_2),
    ("ln10", std::f64::consts::LN_10),
];

const EPSILON: f64 = 1e-10;

/// Recognizes if a value matches a known mathematical constant,
/// a small integer multiple (2-4), or a simple fraction (1/2, 1/3, 1/4).
///
/// Returns the constant annotation string if a match is found.
/// Precedence: exact matches first, then multiples, then fractions.
#[must_use]
pub fn recognize_constant(value: f64) -> Option<String> {
    for &(name, constant) in MATH_CONSTANTS {
        if (value - constant).abs() < EPSILON {
            return Some(name.to_string());
        }
    }

    for multiplier in 2..=4 {
        for &(name, constant) in MATH_CONSTANTS {
            if constant.mul_add(-f64::from(multiplier), value).abs() < EPSILON {
                return Some(format!("{multiplier}{name}"));
            }
        }
    }

    for divisor in 2..=4 {
        for &(name, constant) in MATH_CONSTANTS {
            if (value - constant / f64::from(divisor)).abs() < EPSILON {
                return Some(format!("{name}/{divisor}"));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_all_eight_constants() {
        assert_eq!(MATH_CONSTANTS.len(), 8);
    }

    #[test]
    fn registry_values_match_expected() {
        let find = |name: &str| -> f64 {
            MATH_CONSTANTS
                .iter()
                .find(|(n, _)| *n == name)
                .unwrap_or_else(|| panic!("constant '{name}' not found"))
                .1
        };

        assert!((find("pi") - std::f64::consts::PI).abs() < 1e-15);
        assert!((find("e") - std::f64::consts::E).abs() < 1e-15);
        assert!((find("tau") - std::f64::consts::TAU).abs() < 1e-15);
        assert!((find("phi") - 1.618_033_988_749_895).abs() < 1e-15);
        assert!((find("sqrt2") - std::f64::consts::SQRT_2).abs() < 1e-15);
        assert!((find("sqrt3") - 1.732_050_807_568_877_2).abs() < 1e-15);
        assert!((find("ln2") - std::f64::consts::LN_2).abs() < 1e-15);
        assert!((find("ln10") - std::f64::consts::LN_10).abs() < 1e-15);
    }

    #[test]
    fn recognize_exact_pi() {
        assert_eq!(
            recognize_constant(std::f64::consts::PI),
            Some("pi".to_string())
        );
    }

    #[test]
    fn recognize_exact_e() {
        assert_eq!(
            recognize_constant(std::f64::consts::E),
            Some("e".to_string())
        );
    }

    #[test]
    fn recognize_exact_tau() {
        assert_eq!(
            recognize_constant(std::f64::consts::TAU),
            Some("tau".to_string())
        );
    }

    #[test]
    fn recognize_exact_phi() {
        assert_eq!(
            recognize_constant(1.618_033_988_749_895),
            Some("phi".to_string())
        );
    }

    #[test]
    fn recognize_exact_sqrt2() {
        assert_eq!(
            recognize_constant(std::f64::consts::SQRT_2),
            Some("sqrt2".to_string())
        );
    }

    #[test]
    fn recognize_exact_sqrt3() {
        assert_eq!(
            recognize_constant(1.732_050_807_568_877_2),
            Some("sqrt3".to_string())
        );
    }

    #[test]
    fn recognize_exact_ln2() {
        assert_eq!(
            recognize_constant(std::f64::consts::LN_2),
            Some("ln2".to_string())
        );
    }

    #[test]
    fn recognize_exact_ln10() {
        assert_eq!(
            recognize_constant(std::f64::consts::LN_10),
            Some("ln10".to_string())
        );
    }

    #[test]
    fn recognize_integer_multiple_2pi() {
        assert_eq!(
            recognize_constant(2.0 * std::f64::consts::PI),
            Some("tau".to_string())
        );
    }

    #[test]
    fn recognize_integer_multiple_3e() {
        assert_eq!(
            recognize_constant(3.0 * std::f64::consts::E),
            Some("3e".to_string())
        );
    }

    #[test]
    fn recognize_integer_multiple_4tau() {
        assert_eq!(
            recognize_constant(4.0 * std::f64::consts::TAU),
            Some("4tau".to_string())
        );
    }

    #[test]
    fn recognize_fraction_pi_over_2() {
        assert_eq!(
            recognize_constant(std::f64::consts::PI / 2.0),
            Some("pi/2".to_string())
        );
    }

    #[test]
    fn recognize_fraction_e_over_3() {
        assert_eq!(
            recognize_constant(std::f64::consts::E / 3.0),
            Some("e/3".to_string())
        );
    }

    #[test]
    fn recognize_fraction_phi_over_4() {
        assert_eq!(
            recognize_constant(1.618_033_988_749_895 / 4.0),
            Some("phi/4".to_string())
        );
    }

    #[test]
    fn recognize_no_match_returns_none() {
        assert_eq!(recognize_constant(42.0), None);
    }

    #[test]
    fn recognize_no_match_zero() {
        assert_eq!(recognize_constant(0.0), None);
    }

    #[test]
    fn recognize_epsilon_just_within_tolerance() {
        let value = std::f64::consts::PI + 5e-11;
        assert_eq!(recognize_constant(value), Some("pi".to_string()));
    }

    #[test]
    fn recognize_epsilon_just_outside_tolerance() {
        let value = std::f64::consts::PI + 5e-9;
        assert_eq!(recognize_constant(value), None);
    }

    #[test]
    fn recognize_exact_match_takes_precedence_over_multiple() {
        // tau == 2*pi, but tau should be recognized as exact match "tau" not "2pi"
        assert_eq!(
            recognize_constant(std::f64::consts::TAU),
            Some("tau".to_string())
        );
    }
}
