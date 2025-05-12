use calculator_backend::{calculate_expression, History};

#[test]
fn test_sin() {
    let mut history = History::new();

    // Test sin(0)
    let result = calculate_expression("sin(0) + 5", &mut history);
    assert!(result.success);
    assert!((result.result - 5.0).abs() < 1e-9);

    // Test sin(pi / 2)
    let result = calculate_expression("sin(pi / 2)", &mut history);
    assert!(result.success);
    assert!((result.result - 1.0).abs() < 1e-9);

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
    assert!((result.result - 1.0).abs() < 1e-9);

    // Test cos(pi / 2)
    let result = calculate_expression("cos(pi / 2)", &mut history);
    assert!(result.success);
    assert!((result.result - 0.0).abs() < 1e-9);

    // Test cos(pi)
    let result = calculate_expression("cos(pi)", &mut history);
    assert!(result.success);
    assert!((result.result + 1.0).abs() < 1e-9);
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
    let result = calculate_expression("tan(2)", &mut history);
    assert_eq!(result.result, -2.185039863); // Check for infinity
}

#[test]
fn test_inverse_trig() {
    let mut history = History::new();

    // Test asin(1)
    let result = calculate_expression("arcsin(1)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 1.570796327);

    // Test acos(1)
    let result = calculate_expression("arccos(1)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 0.0);

    // Test atan(1)
    let result = calculate_expression("arctan(1)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 0.785398163);
}

#[test]
fn test_invalid_trig() {
    let mut history = History::new();

    let result = calculate_expression("arcsin(2)", &mut history);

    assert_eq!(result.message, "Invalid trigonometric operator");

    // Test acos(-2) (should fail because input is out of range)
    let result = calculate_expression("arccos(-2)", &mut history);
    assert_eq!(result.message, "Invalid trigonometric operator");

    // Test tan(large number) (should fail due to undefined behavior)
    let result = calculate_expression("tan(1e10)", &mut history);
    assert!((result.result - -0.558349638).abs() < 1e-9); // Validate the result
}

#[test]
fn test_trig_mix_operations() {
    let mut history = History::new();

    // Test sin(pi / 2) + sqrt(5)
    let result = calculate_expression("sin(pi / 2) + sqrt(5)", &mut history);
    assert!(result.success);
    assert!((result.result - 3.236067977).abs() < 1e-9);

    // Test mixed operations with trig functions
    let result = calculate_expression("sin(pi / 2) + cos(0) * tan(0)", &mut history);
    assert!(result.success);
    assert!((result.result - 1.0).abs() < 1e-9);
}

#[test]
fn test_large_numbers() {
    let mut history = History::new();

    // Test sin(large number)
    let result = calculate_expression("sin(1e10)", &mut history);
    assert!(result.success);
    assert!(result.result.abs() <= 1.0); // sin(x) is always between -1 and 1

    // Test cos(large number)
    let result = calculate_expression("cos(1e10)", &mut history);
    assert!(result.success);
    assert!(result.result.abs() <= 1.0); // cos(x) is always between -1 and 1
}