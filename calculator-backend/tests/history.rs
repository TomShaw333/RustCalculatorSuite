#[cfg(test)]
mod tests {
    use calculator_backend::calculate_expression;
    use calculator_backend::History;

    #[test]
    fn test_history_logging() {
        let mut history = History::new();

        // Successful calculation
        let result = calculate_expression("1 + 2", &mut history);
        assert!(result.success);
        assert_eq!(result.result, 3.0);

        // Failed calculation
        let result = calculate_expression("1 / 0", &mut history);
        assert!(!result.success);

        // Check history
        let entries = history.get_history();
        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].input, "1 + 2");
        assert_eq!(entries[0].result, Some(3.0));
        assert!(entries[0].error_message.is_none());

        assert_eq!(entries[1].input, "1 / 0");
        assert!(entries[1].result.is_none());
        assert_eq!(entries[1].error_message.as_ref().unwrap(), "Division by zero");
    }

    #[test]
    fn test_ans_functionality() {
        let mut history = History::new();

        // Perform a calculation
        let result = calculate_expression("5 + 5", &mut history);
        assert!(result.success);
        assert_eq!(result.result, 10.0);

        // Use "ans" in the next calculation
        let result = calculate_expression("ans + 5", &mut history);
        assert!(result.success);
        assert_eq!(result.result, 15.0);

        // Use "ans" again
        let result = calculate_expression("ans * 2", &mut history);
        assert!(result.success);
        assert_eq!(result.result, 30.0);
    }
}