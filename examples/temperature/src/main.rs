
fn fahrenheit_to_celsius_v1(temperature: f64) -> f64 {
    return (temperature - 32.0) * (5.0 / 9.0);
}

fn fahrenheit_to_celsius_v2(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0 / 9.0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_v1_with_32_f() {
        assert_eq!(fahrenheit_to_celsius_v1(32f64), 0.0);
    }

    #[test]
    fn test_v2_with_32_f() {
        assert_eq!(fahrenheit_to_celsius_v2(32f64), 0.0);
    }

    #[test]
    fn test_v1_with_68_f() {
        assert_eq!(fahrenheit_to_celsius_v1(68.0), 20.0);
    }

    #[test]
    fn test_v2_with68_f() {
        assert_eq!(fahrenheit_to_celsius_v2(68.0), 20.0);
    }

}
