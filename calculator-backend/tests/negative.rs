use calculator_backend::{calculate_expression, convert_rpn, History};

#[test]
fn test_negative_addition() {
    let mut history = History::new();
    let result = calculate_expression("-1 + 2", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 1.0);
}

#[test]
fn test_negative_multiplication() {
    let mut history = History::new();
    let result = calculate_expression("-3 * -4", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 12.0);
}

#[test]
fn test_negative_division() {
    let mut history = History::new();
    let result = calculate_expression("-6 / 2", &mut history);
    assert!(result.success);
    assert_eq!(result.result, -3.0);
}

#[test]
fn test_negative_parentheses() {
    let mut history = History::new();
    let result = calculate_expression("(-1 + 2) * -3", &mut history);
    assert!(result.success);
    assert_eq!(result.result, -3.0);
}

#[test]
fn test_consecutive_negatives() {
    let mut history = History::new();
    let result = calculate_expression("-1 - -2 - -3", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 4.0);
}

#[test]
fn test_negative_exponent() {
    let mut history = History::new();
    let result = calculate_expression("2 ^ -2", &mut history);
    assert!(result.success);
    assert_eq!(result.result, 0.25);
}

#[test]
fn test_complex_negative_expression() {
    let mut history = History::new();
    let result = calculate_expression("(-2 * -3) / (-6 + 2)", &mut history);
    assert!(result.success);
    assert_eq!(result.result, -1.5);
}

#[test]
fn test_rpn_negative_numbers() {
    let result = convert_rpn("-1 -2 +".to_string());
    assert!(result.success);
    assert_eq!(result.infix_expression, "-1 + -2");
}
