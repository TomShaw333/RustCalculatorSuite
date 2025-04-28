use calculator_backend::{calculate_expression, History};

#[test]
fn test_sin() {
    let mut history = History::new();

    // Test sin(0)
    let result = calculate_expression("sin(0)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 0.0);

    // Test sin(pi / 2)
    let result = calculate_expression("sin(pi / 2)", &mut history);
    assert!(result.success);
    assert!((result.result - 1.0).abs() < 1e-9); // Allow for floating-point precision

    // Test sin(pi)
    let result = calculate_expression("sin(pi)", &mut history);
    assert!(result.success);
    assert!((result.result - 0.0).abs() < 1e-9);
}

#[test]
fn test_cos() {
    let mut history = History::new();

    // Test cos(0)
    let result = calculate_expression("cos(0)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 1.0);

    // Test cos(pi / 2)
    let result = calculate_expression("cos(pi / 2)", &mut history);
    assert!(result.success);
    assert!((result.result - 0.0).abs() < 1e-9);

    // Test cos(pi)
    let result = calculate_expression("cos(pi)", &mut history);
    assert!(result.success);
    assert!((result.result + 1.0).abs() < 1e-9); // cos(pi) = -1
}

#[test]
fn test_tan() {
    let mut history = History::new();

    // Test tan(0)
    let result = calculate_expression("tan(0)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 0.0);

    // Test tan(pi / 4)
    let result = calculate_expression("tan(pi / 4)", &mut history);
    assert!(result.success);
    assert!((result.result - 1.0).abs() < 1e-9);

    // Test tan(pi / 2) (should fail due to undefined value)
    let result = calculate_expression("tan(pi / 2)", &mut history);
    assert!(!result.success);
    assert_eq!(result.message, "Invalid operator");
}

#[test]
fn test_inverse_trig() {
    let mut history = History::new();

    // Test asin(1)
    let result = calculate_expression("asin(1)", &mut history);
    assert!(result.success);
    assert!((result.result - (std::f64::consts::PI / 2.0)).abs() < 1e-9);

    // Test acos(1)
    let result = calculate_expression("acos(1)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 0.0);

    // Test atan(1)
    let result = calculate_expression("atan(1)", &mut history);
    assert!(result.success);
    assert!((result.result - (std::f64::consts::PI / 4.0)).abs() < 1e-9);
}

#[test]
fn test_invalid_trig() {
    let mut history = History::new();

    // Test asin(2) (should fail because input is out of range)
    let result = calculate_expression("asin(2)", &mut history);
    assert!(!result.success);
    assert_eq!(result.message, "Invalid operator");

    // Test acos(-2) (should fail because input is out of range)
    let result = calculate_expression("acos(-2)", &mut history);
    assert!(!result.success);
    assert_eq!(result.message, "Invalid operator");
}

#[test]
fn test_trig_mix_operations() {
    let mut history = History::new();

    let result = calculate_expression("sin(pi /2) + sqrt(5)", &mut history);
    assert_eq!(result.result, 3.236067977);

    // Test mixed operations with trig functions
    let result = calculate_expression("sin(pi/2) + cos(0) * tan(0)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 1.0);
}

#[test]
fn test_logarithms() {
    let mut history = History::new();

    // Test log(10)
    let result = calculate_expression("log(10)", &mut history);
    assert_eq!(result.result, 1.0);

    // Test log(100)
    let result = calculate_expression("log(100)", &mut history);
    assert_eq!(result.result, 2.0);

    // Test ln(e)
    let result = calculate_expression("ln(e)", &mut history);
    assert_eq!(result.result, 1.0);

    // Test ln(1)
    let result = calculate_expression("ln(1)", &mut history);
    assert_eq!(result.result, 0.0);

    // Test invalid log(-1)
    let result = calculate_expression("log(-1)", &mut history);
    assert_eq!(result.message, "Log error");

    // Test invalid ln(0)
    let result = calculate_expression("ln(0)", &mut history);
    assert_eq!(result.message, "Natural logarithm error");
}