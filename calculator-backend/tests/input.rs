use calculator_backend::{calculate_expression, History};

#[test]
fn test_tokenize_lowercase() {
    let mut history = History::new();
    let result = calculate_expression("SIN(PI/2) + LOG(10)", &mut history);

    assert_eq!(result.result, 2.0);
}

#[test]
fn test_tokenize_case_insensitivity() {
    let mut history = History::new();

    // Test with uppercase input
    let result = calculate_expression("COS(PI/2)", &mut history);
    assert_eq!(result.result, 0.0);
}