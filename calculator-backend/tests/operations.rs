use calculator_backend::{calculate_expression, History};

#[test]
fn test_factorial() {
    let mut history = History::new();

    // Test factorial of a positive number
    let result = calculate_expression("5 !", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 120.0);

    // Test factorial of 0
    let result = calculate_expression("0 !", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 1.0);

    // Test factorial of a negative number (should fail)
    let result = calculate_expression("-5 !", &mut history);
    assert!(!result.success);
    assert_eq!(result.message, "Factorial error");

    // Test factorial of a non-integer
    let result = calculate_expression("5.5!", &mut history);
    assert!(!result.success);
    assert_eq!(result.message, "Factorial error");

    // Test factorial with a second operand
    let result = calculate_expression("5! + 2", &mut history);
    assert_eq!(result.result, 122.0);

}
#[test]
fn test_square_root() {
    let mut history = History::new();

    // Test square root of a positive number
    let result = calculate_expression("sqrt 16", &mut history);
    assert_eq!(result.result, 4.0);

    // Test square root of 0
    let result = calculate_expression("sqrt 0", &mut history);
    assert_eq!(result.result, 0.0);

    // Test square root of a negative number (should fail)
    let result = calculate_expression("√ -16", &mut history);
    assert_eq!(result.message, "Square root error");

    // Test square root with additional operations
    let result = calculate_expression("√ 16 + 4", &mut history);
    assert_eq!(result.result, 8.0);

    let result = calculate_expression("√ 25 * 2", &mut history);
    assert_eq!(result.result, 10.0);

    let result = calculate_expression("√ 36 / 2", &mut history);
    assert_eq!(result.result, 3.0);
}

#[test]
fn test_combined_operations() {
    let mut history = History::new();

    // Test factorial and square root combined
    let result = calculate_expression("√ 25 + 5!", &mut history);
    assert_eq!(result.result, 125.0);

    let result = calculate_expression("5! - √ 16", &mut history);
    assert_eq!(result.result, 116.0);

    let result = calculate_expression("√ 36 + 5! / 2", &mut history);
    assert_eq!(result.result, 66.0);

    let result = calculate_expression("5! + 2", &mut history);
    assert_eq!(result.result, 122.0);
    
    let result = calculate_expression("sqrt 25 + 2", &mut history);
    assert_eq!(result.result, 7.0);
}

#[test]
fn test_invalid_expressions() {
    let mut history = History::new();

    // Test invalid square root usage
    let result = calculate_expression("√", &mut history);
    assert!(!result.success);
    assert_eq!(result.message, "Invalid operator");

    let result = calculate_expression("√ + 5", &mut history);
    assert!(!result.success);
    assert_eq!(result.message, "Invalid operator");

    let result = calculate_expression("5 ! +", &mut history);
    assert_eq!(result.message, "Stack underflow - invalid expression");
}