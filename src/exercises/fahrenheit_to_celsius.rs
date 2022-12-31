pub fn fahrenheit_to_celcius(value: f32) -> f32 {
    (value - 32.0) * 5.0 / 9.0
}

#[test]
fn fahrenheit_to_celcius_test() {
    assert_eq!(0.0, fahrenheit_to_celcius(32.0));
}
