// ./c_library/calculator.c
// This file contains the implementation of a calculator that can evaluate expressions in Reverse Polish Notation (RPN).
// It supports various arithmetic operations, including addition, subtraction, multiplication, division, exponentiation, and more.
// The calculator also handles variables, error codes, and conversion between RPN and infix notation.

#include <stdio.h>
#include <math.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <ctype.h>

// Error codes
#define SUCCESS 0
#define DIVISION_BY_ZERO 1
#define INVALID_OPERATOR 2
#define STACK_UNDERFLOW 3
#define MEMORY_ERROR 4
#define UNDEFINED_VARIABLE 5
#define STACK_MAXIMUM 6
#define EXPR_LENGHT_MAXIMUM 7
#define FACTORIAL_ERROR 8
#define SQUARE_ROOT_INVALID_OPERATOR 9
#define LOG_ERROR 10
#define LN_ERROR 11
#define TAN_INVALID_OPERATOR 12
#define INVALID_TRIG_OPERATOR 13

// Maximum size for expression and operator arrays
#define MAX_STACK_SIZE 100
#define MAX_VARIABLES 100
#define MAX_EXPR_LENGTH 1000

#ifndef M_PI
#    define M_PI 3.14159265358979323846
#endif

#ifndef M_E
#    define M_E 2.71828182845904523536
#endif

/* Represents a result of a calculation, containing both the value obtained from 
   the calculation and an associated error code. */
typedef struct {
    double value; //The calculated value
    int error_code; //Code indicating success(0) or error
} CalculationResult;

/*Represents a expression in Reverse Polish Notation, containing the expression and the 
  length of the expression. */
typedef struct {
    const char** expression;
    size_t length;
} ReversePolishExpression;

/*Represents a named quantity or property with a descriptive name and a numerical value. */
typedef struct {
    char name[50];
    double value;
} Variable;

/*Represents a the result of converting an expression from one form to another. */
typedef struct {
    char expression[MAX_EXPR_LENGTH];
    int error_code;
} ConversionResult;

/**
 * @brief Array to store variables.
 *
 * This static array holds the defined variables, each with a name and a value.
 * The size of the array is limited by MAX_VARIABLES.
 */
static Variable variables[MAX_VARIABLES];

/**
 * @breif Counter for the number of variables stored. 
 * 
 * This static int keeps track of the number of variables stored in the 'variables' array. 
 */
static int variable_count = 0;

/**
 * @brief Initializes default variables.
 *
 * This function initializes the `variables` array with default values,
 * such as 'pi' and 'e'. It also resets the `variable_count` to 0 before
 * adding the default variables.
 */
static void init_default_variables() {
    // Add any default variables here
    variable_count = 0;
    variables[variable_count++] = (Variable){"pi", M_PI};
    variables[variable_count++] = (Variable){"e", M_E};
}

/**car
 * @breif Returns the value of a variable by its name 
 * 
 * Searches the 'variables' array for the variable of given name if found it returns the value
 * of the variable, if not it sets the 'error_code' to 'UNDEFINED_VARIABLE' and it returns 0.0.
 * 
 * @param name       The name of the variable being retrieved 
 * @param error_code A pointer to an integer where an error code can be stored.
 *                   -   If the variable is found, the error code is not modified.
 *                   -   If the variable is not found, the error code is set to UNDEFINED_VARIABLE.
 * @return           The value of the variable, 0.0 if not found. 
 */
static double get_variable_value(const char* name, int* error_code) {
    printf("Looking up variable: %s\n", name);
    for (int i = 0; i < variable_count; i++) {
        if (strcmp(variables[i].name, name) == 0) {
            printf("Found variable %s = %f\n", name, variables[i].value);
            return variables[i].value;
        }
    }
    printf("Variable not found: %s\n", name);
    *error_code = UNDEFINED_VARIABLE;
    return 0.0;
}

/**
 * @brief Checks if a token is a valid arithmetic operator.
 *
 * Determines whether a given token represents one of the supported
 * arithmetic operators. It checks if the token matches one of the
 * allowed operators, including:
 * - Basic arithmetic: '+', '-', '*', '/', and '^' (power).
 * - Factorial: '!' (unary operator, only one operand is used).
 * - Square root: "sqrt" (unary operator).
 * - Trigonometric functions: "sin", "cos", "tan", "arcsin", "arccos", "arctan" (unary operators).
 * - Logarithmic functions:
 *   - "log" (base-10 logarithm, unary operator).
 *   - "ln" (natural logarithm, unary operator).
 * - Special constants or functions:
 *   - "ans" (refers to the result of the last calculation, unary operator).
 *
 * @param token A string representing a single token from an expression.
 *              - It is expected to be a null-terminated C-style string.
 * @return True if the token is an operator, false otherwise.
 */
static bool is_operator(const char* token) {
    return (
        strcmp(token, "+") == 0 ||
        strcmp(token, "-") == 0 ||
        strcmp(token, "*") == 0 ||
        strcmp(token, "/") == 0 ||
        strcmp(token, "^") == 0 ||
        strcmp(token, "!") == 0 ||
        strcmp(token, "sqrt") == 0 ||
        strcmp(token, "sin") == 0 ||
        strcmp(token, "cos") == 0 ||
        strcmp(token, "tan") == 0 ||
        strcmp(token, "arcsin") == 0 ||
        strcmp(token, "arccos") == 0 ||
        strcmp(token, "arctan") == 0 ||
        strcmp(token, "log") == 0 ||
        strcmp(token, "ln") == 0 ||
        strcmp(token, "ans") == 0
    );
}

/**
 * @brief Checks if a token represents a valid floating point number.
 * 
 * Determines whether a given token can be interpreted as a
 * valid floating-point number. It uses the `strtod` function for the
 * conversion and checks for successful conversion. A valid number can
 * optionally be followed by a newline character ('\n')
 * 
 *@param token A char representing a single token from an expression.
 *              It is expected to be a null-terminated C-style string.
 * @return True if token is an number, false otherwise. 
 */
static bool is_number(const char* token) {
    char* endptr;
    strtod(token, &endptr);
    return *endptr == '\0' || (*endptr == '\n' && endptr[1] == '\0');
}

/**
 * @brief Computes the factorial of a non-negative integer.
 *
 * This function calculates the factorial of a given non-negative integer `n`.
 * If `n` is negative or not an integer, the function sets the `error_code` to
 * `FACTORIAL_ERROR` and returns NaN. The factorial of 0 or 1 is 1.
 *
 * @param n          The number for which the factorial is to be calculated.
 *                   It must be a non-negative integer.
 * @param error_code A pointer to an integer where an error code can be stored.
 *                   - If the calculation is successful, the error code is not modified.
 *                   - If `n` is negative or not an integer, the error code is set to `FACTORIAL_ERROR`.
 * @return           The factorial of `n` if successful, or NaN if an error occurs.
 */
static double factorial(double n, int* error_code) {
    if (n < 0) {
        *error_code = FACTORIAL_ERROR; // Set error code for invalid factorial
        printf("Factorial error: Factorial is undefined for negative numbers\n");
        return NAN; // Return NaN to indicate an error
    }
    if (n != (int)n) {
        *error_code = FACTORIAL_ERROR; // Set error code for non-integer factorial
        printf("Factorial error: Factorial is undefined for non-integer values\n");
        return NAN;
    }
    if (n == 0 || n == 1) {
        return 1.0;
    }
    double result = 1.0;
    for (int i = 1; i <= (int)n; i++) {
        result *= i;
    }
    return result;
}


/**
 * @brief Rounds a double to 9 decimal places.
 *
 * This function rounds the given double value to 9 decimal places.
 *
 * @param value The double value to be rounded.
 * @return The rounded value.
 */
static double round_to_9_decimals(double value) {
    return round(value * 1e9) / 1e9;
}

/**
 * @brief Applies an arithmetic operation to two operands.
 *
 * This function performs an arithmetic operation on two double-precision
 * floating-point operands (`a` and `b`) based on the specified operator (`op`).
 * It supports the following operators:
 * - Basic arithmetic: '+', '-', '*', '/', and '^' (power).
 * - Factorial: '!' (unary operator, only one operand is used).
 * - Square root: "sqrt" (unary operator).
 * - Trigonometric functions: "sin", "cos", "tan" (unary operators).
 * - Logarithmic functions:
 *   - "log" (base-10 logarithm, unary operator).
 *   - "ln" (natural logarithm, unary operator).
 * - Special constants or functions:
 *   - "ans" (refers to the result of the last calculation, unary operator).
 *
 * The result of the operation is returned. If an error occurs, the error code
 * is updated, and the function returns 0.0.
 *
 * @param op         A string representing the arithmetic operator.
 *                   Valid operators include '+', '-', '*', '/', '^', '!', "sqrt",
 *                   "sin", "cos", "tan", "log", "ln", and "ans".
 * @param a          The first operand (double). For unary operators, this is the only operand.
 * @param b          The second operand (double). Not used for unary operators.
 * @param error_code A pointer to an integer where an error code can be stored.
 *                      - If the operation is successful, the error code is not modified.
 *                      - If division by zero occurs ('a / 0'), the error code is set to
 *                        DIVISION_BY_ZERO.
 *                      - If the operator is not recognized, the error code is set to
 *                        INVALID_OPERATOR.
 *                      - If a logarithmic function is applied to a non-positive number,
 *                        the error code is set to LOG_ERROR or LN_ERROR.
 *                      - If "sqrt" is applied to a negative number, the error code is set to
 *                        SQUARE_ROOT_INVALID_OPERATOR.
 * @return           The result of the arithmetic operation (double).
 *                      - Returns the calculated result if the operation is successful.
 *                      - Returns 0.0 if an error occurs (e.g., division by zero, invalid operator).
 */
static double apply_operator(const char* op, double a, double b, int* error_code) {
    double result;

    // Handle string-based unary operator first
    if (strcmp(op, "sqrt") == 0) {
        if (a < 0) {
            *error_code = SQUARE_ROOT_INVALID_OPERATOR;
            printf("Square root error: sqrt of negative number\n");
            return 0;
        }
        result = sqrt(a);
        result = round_to_9_decimals(result);
        return result;
    }

    if (strcmp(op, "sin") == 0){
        result = sin(a);
        result = round_to_9_decimals(result);
        return result;
    }

    if (strcmp(op, "cos") == 0){
        result = cos(a);
        result = round_to_9_decimals(result);
        return result;
    }

    if (strcmp(op, "tan") == 0) {
        // Check for undefined values of tan (odd multiples of π/2)
        if (fmod(fabs(a), M_PI) == M_PI / 2) {
            *error_code = TAN_INVALID_OPERATOR;
            printf("Undefined value for tan at odd multiples of π/2\n");
            return 0;
        }
        result = tan(a);
        result = round_to_9_decimals(result);
        return result;
    }

    if (strcmp(op, "arcsin") == 0) {
        if (a < -1 || a > 1) {
            *error_code = INVALID_TRIG_OPERATOR;
            printf("Invalid input for asin\n");
            return 0;
        }
        result = asin(a);
        result = round_to_9_decimals(result);
        return result;
    }
    
    if (strcmp(op, "arccos") == 0) {
        if (a < -1 || a > 1) {
            *error_code = INVALID_TRIG_OPERATOR;
            printf("Invalid input for acos\n");
            return 0;
        }
        result = acos(a);
        result = round_to_9_decimals(result);
        return result;
    }
    
    if (strcmp(op, "arctan") == 0) {
        result = atan(a);
        result = round_to_9_decimals(result);
        return result;
    }

    if (strcmp(op, "log") == 0) {  // Base-10 logarithm
        if (a <= 0) {
            *error_code = LOG_ERROR;
            printf("Logarithm error: log of non-positive number\n");
            return 0;
        }
        result = log10(a);
        result = round_to_9_decimals(result);
        return result;
    }

    if (strcmp(op, "ln") == 0) {  // Natural logarithm
        if (a <= 0) {
            *error_code = LN_ERROR;
            printf("Natural logarithm error: ln of non-positive number\n");
            return 0;
        }
        result = log(a);
        result = round_to_9_decimals(result);
        return result;
    }
    
    // Handle other operators with a switch
    if (strlen(op) == 1) {
        char c = op[0];
        switch (c) {
            case '+': result = a + b; break;
            case '-': result = a - b; break;
            case '*': result = a * b; break;
            case '/':
                if (b == 0) {
                    *error_code = DIVISION_BY_ZERO;
                    printf("Division by zero error\n");
                    return 0;
                }
                result = a / b;
                break;
            case '^': result = pow(a, b); break;
            case '!':
                if (b != 0) {
                    *error_code = INVALID_OPERATOR;
                    printf("Factorial takes one operand only\n");
                    return 0;
                }
                result = factorial(a, error_code);
                if (*error_code != SUCCESS) return 0;
                break;
            default:
                *error_code = INVALID_OPERATOR;
                printf("Unknown operator: %c\n", c);
                return 0;
        }
        result = round_to_9_decimals(result);
        return result;
    }

    result = round_to_9_decimals(result);
    *error_code = INVALID_OPERATOR;
    printf("Invalid operator (not single char or known string): %s\n", op);
    return 0;
}

/**
 * @brief Evaluates a Reverse Polish Notation (RPN) expression.
 *
 * This function evaluates an expression given in Reverse Polish Notation (RPN).
 * It uses a stack to store operands and intermediate results. The function
 * handles arithmetic operations, variable lookups, and error conditions
 * such as invalid expressions, stack underflow/overflow, division by zero,
 * and invalid operators.
 *
 * @param rpn        A pointer to a ReversePolishExpression structure representing
 *                   the RPN expression to be evaluated. The structure contains the
 *                   expression as an array of string tokens and the length of the
 *                   expression.
 * @param error_code A pointer to an integer where an error code can be stored.
 *                      -   If the evaluation is successful, the result's 'value' field
 *                           holds the calculated result, and the 'error_code' field is set
 *                           to SUCCESS (0).
 *                      -   If an error occurs, the result's 'value' field is 0.0, and the
 *                          'error_code' field is set to the corresponding error code:
 *                          MEMORY_ERROR, STACK_UNDERFLOW, STACK_OVERFLOW,
 *                          DIVISION_BY_ZERO, or INVALID_OPERATOR.
 * @return           The result of the arithmetic operation (double).
 *                      -   Returns the calculated result if the operation is successful.
 *                      -   Returns 0.0 if an error occurs (division by zero or invalid operator).
 */
CalculationResult evaluate_rpn(const ReversePolishExpression* rpn) {
    CalculationResult result = {0.0, SUCCESS};
    
    printf("Starting RPN evaluation\n");
    
    if (!rpn || !rpn->expression) {
        printf("Memory error: NULL pointer received\n");
        result.error_code = MEMORY_ERROR;
        return result;
    }

    printf("Expression length: %zu\n", rpn->length);

    double stack[MAX_STACK_SIZE];
    int stack_top = -1;

    init_default_variables();

    for (size_t i = 0; i < rpn->length; i++) {
        const char* token = rpn->expression[i];
        printf("Processing token: %s\n", token);
        
        if (is_operator(token)) {
            if (token[0] == '!' || strcmp(token, "sqrt") == 0 || strcmp(token, "sin") == 0 || strcmp(token,"cos") == 0 || strcmp(token, "tan") == 0
            || strcmp(token, "arcsin") == 0 || strcmp(token, "arccos") == 0 || strcmp(token, "arctan") == 0 || strcmp(token, "log") == 0 || strcmp(token, "ln") == 0) { 
                if (stack_top < 0) {
                    printf("Operator without operand\n");
                    result.error_code = INVALID_OPERATOR;
                    return result;
                }
        
                double a = stack[stack_top--]; // Pop one operand
                int error_code = SUCCESS;
                double op_result = apply_operator(token, a, 0, &error_code); // Pass 0 as the second operand
        
                if (error_code != SUCCESS) {
                    result.error_code = error_code;
                    return result;
                }
        
                stack_top++;
                stack[stack_top] = op_result; // Push the result back onto the stack
                printf("Pushed result: %.9f\n", stack[stack_top]);
            } else { // Handle binary operators
                if (stack_top < 1) {
                    printf("Stack underflow error\n");
                    result.error_code = STACK_UNDERFLOW;
                    return result;
                }
        
                double b = stack[stack_top--];
                double a = stack[stack_top--];
        
                int error_code = SUCCESS;
                double op_result = apply_operator(token, a, b, &error_code);
        
                if (error_code != SUCCESS) {
                    result.error_code = error_code;
                    return result;
                }
        
                stack_top++;
                stack[stack_top] = op_result;
                printf("Pushed result: %.9f\n", stack[stack_top]);
            }
        } else if (is_number(token)) {
            stack_top++;
            if (stack_top >= MAX_STACK_SIZE) {
                printf("Stack overflow error\n");
                result.error_code = STACK_MAXIMUM;
                return result;
            }
            stack[stack_top] = atof(token);
            printf("Pushed number: %.9f\n", stack[stack_top]);

            if (rpn->length > MAX_EXPR_LENGTH) {
                printf("Expression length exceeded\n");
                result.error_code = EXPR_LENGHT_MAXIMUM;
                return result;
            }
        } else {
            // Must be a variable
            int error_code = SUCCESS;
            double value = get_variable_value(token, &error_code);
            if (error_code != SUCCESS) {
                result.error_code = error_code;
                return result;
            }
            stack_top++;
            stack[stack_top] = value;
            printf("Pushed variable value: %.9f\n", stack[stack_top]);
        }
    }

    if (stack_top != 0) {
        printf("Invalid expression: stack not empty\n");
        result.error_code = STACK_UNDERFLOW;
        return result;
    }

    result.value = stack[0];
    printf("Final result: %.9f\n", result.value);
    return result;
}

/**
 * @brief Foreign Function Interface (FFI) wrapper for RPN calculation.
 * 
 * It takes a ReversePolishExpression as input,
 * calls the evaluate_rpn function to perform the calculation, and
 * returns the CalculationResult. It also includes logging via printf
 * statements for debugging and tracing purposes.
 * 
 * @param rpn A pointer to a ReversePolishExpression structure representing
 *            the RPN expression to be evaluated. The structure contains the
 *            expression as an array of string tokens and the length of the
 *            expression.
 * 
 * @return    A CalculationResult structure containing the result of the
 *            evaluation and an error code.
 *              -   The 'value' field holds the calculated result if the evaluation
 *                  is successful.
 *              -   The 'error_code' field indicates the success or failure of the
 *                  operation.
 */
CalculationResult calculate_rpn(const ReversePolishExpression* rpn) {
    printf("FFI: calculate_rpn called\n");
    CalculationResult result = evaluate_rpn(rpn);
    printf("FFI: returning result value=%f, error_code=%d\n", result.value, result.error_code);
    return result;
}

/**
 * @brief Determines if parentheses are needed around an expression.
 *
 * Checks if an expression requires parentheses based on the
 * given operator's precedence. It checks for existing parentheses at the
 * beginning and end of the expression and then iterates through the
 * expression to identify operators with lower precedence.
 *
 * @param expr The expression string to check. It is expected to be a
 *             null-terminated C-style string.
 * @param op   The operator character to consider for precedence.
 * @return     True if parentheses are needed, False otherwise.
 *                  -   False if the expression's length is less than or equal to 1.
 *                  -   False if the expression is already enclosed in parentheses.
 *                  -   True if the expression contains operators of lower precedence
 *                  based on the given operator.
 */
static bool needs_parentheses(const char* expr, char op) {
    if (strlen(expr) <= 1) return false;
    
    // Check first and last characters for existing parentheses
    if (expr[0] == '(' && expr[strlen(expr)-1] == ')') return false;
    
    // Check if the expression contains operators of lower precedence
    for (size_t i = 0; i < strlen(expr); i++) {
        char c = expr[i];
        if (op == '!' || op == '√') return true;
        if ((op == '*' || op == '/') && (c == '+' || c == '-')) return true;
        if (op == '^' && (c == '*' || c == '/')) return true;
    }
    
    return false;
}
/**
 * @brief Converts a Reverse Polish Notation (RPN) expression to Infix notation.
 * 
 * It takes a ReversePolishExpression as input, calls the internal
 * conversion logic, and returns the result as a ConversionResult structure.
 * It also handles memory management for the resulting infix expression.
 * 
 * @param rpn A pointer to a ReversePolishExpression structure representing
 *            the RPN expression to be converted. The structure contains the
 *            expression as an array of string tokens and the length of the
 *            expression.
 * @return    A ConversionResult structure containing the converted infix
 *            expression and an error code.
 *              -   If the conversion is successful, the 'expression' field holds
 *                  the converted infix expression, and the 'error_code' field is
 *                  set to SUCCESS (0). The caller is responsible for freeing the
 *                  memory allocated for the 'expression' field.
 *              -   If an error occurs, the 'expression' field may contain a partial
 *                  result or an empty string, and the 'error_code' field is set to
 *                  the corresponding error code: MEMORY_ERROR or STACK_UNDERFLOW.
 */
ConversionResult rpn_to_infix(const ReversePolishExpression* rpn) {
    ConversionResult result = {"", SUCCESS};
    
    if (!rpn || !rpn->expression) {
        result.error_code = MEMORY_ERROR;
        return result;
    }

    printf("Starting RPN to infix conversion\n");
    printf("Input expression length: %zu\n", rpn->length);

    char* stack[MAX_STACK_SIZE];
    int stack_top = -1;
    
    for (size_t i = 0; i < rpn->length; i++) {
        const char* token = rpn->expression[i];
        printf("Processing token: %s\n", token);
        
        if (!is_operator(token)) {
            // Push operand or variable
            char* expr = malloc(strlen(token) + 1);
            if (!expr) {
                printf("Memory allocation failed\n");
                result.error_code = MEMORY_ERROR;
                // Clean up stack
                while (stack_top >= 0) {
                    free(stack[stack_top--]);
                }
                return result;
            }
            strcpy(expr, token);
            stack[++stack_top] = expr;
            printf("Pushed operand/variable: %s\n", expr);
        } else {
            // Process operator
            if (stack_top < 1) {
                printf("Stack underflow - not enough operands for operator %s\n", token);
                result.error_code = STACK_UNDERFLOW;
                // Clean up stack
                while (stack_top >= 0) {
                    free(stack[stack_top--]);
                }
                return result;
            }
            
            char* b = stack[stack_top--];
            char* a = stack[stack_top--];
            
            printf("Combining: %s %c %s\n", a, token[0], b);
            
            // Allocate space for new expression
            size_t new_len = strlen(a) + strlen(b) + 10; // Extra space for operators and parentheses
            char* expr = malloc(new_len);
            if (!expr) {
                printf("Memory allocation failed\n");
                free(a);
                free(b);
                result.error_code = MEMORY_ERROR;
                // Clean up stack
                while (stack_top >= 0) {
                    free(stack[stack_top--]);
                }
                return result;
            }
            
            // Check if operands need parentheses
            bool need_parens_a = needs_parentheses(a, token[0]);
            bool need_parens_b = needs_parentheses(b, token[0]);
            
            snprintf(expr, new_len, "%s%s%s %c %s%s%s",
                    need_parens_a ? "(" : "",
                    a,
                    need_parens_a ? ")" : "",
                    token[0],
                    need_parens_b ? "(" : "",
                    b,
                    need_parens_b ? ")" : "");
            
            printf("Created expression: %s\n", expr);
            
            free(a);
            free(b);
            stack[++stack_top] = expr;
        }
    }
    
    if (stack_top != 0) {
        printf("Invalid expression: stack not empty (stack_top = %d)\n", stack_top);
        result.error_code = STACK_UNDERFLOW;
        // Clean up stack
        while (stack_top >= 0) {
            free(stack[stack_top--]);
        }
        return result;
    }
    
    // Copy final expression to result
    strncpy(result.expression, stack[0], MAX_EXPR_LENGTH - 1);
    result.expression[MAX_EXPR_LENGTH - 1] = '\0';
    printf("Final expression: %s\n", result.expression);
    free(stack[0]);
    
    return result;
}

/**
 *  @brief Foreign Function Interface (FFI) wrapper for converting RPN expressions to Infix.
 * 
 *  It takes a ReversePolishExpression as input, calls the internal
 * conversion logic, and returns the result as a ConversionResult structure.
 * It also handles memory management for the resulting infix expression.
 * 
 * @param rpn A pointer to a ReversePolishExpression structure representing
 *            the RPN expression to be converted. The structure contains the
 *            expression as an array of string tokens and the length of the
 *            expression.
 * 
 * @return    A ConversionResult structure containing the converted infix
 *            expression and an error code.
 *              -   If the conversion is successful, the 'expression' field holds
 *                  the converted infix expression, and the 'error_code' field is
 *                  set to SUCCESS (0). The caller is responsible for freeing the
 *                  memory allocated for the 'expression' field.
 *              -   If an error occurs, the 'expression' field may contain a partial
 *                  result or an empty string, and the 'error_code' field is set to
 *                  the corresponding error code: MEMORY_ERROR or STACK_UNDERFLOW.
 */
ConversionResult convert_rpn_to_infix(const ReversePolishExpression* rpn) {
    printf("FFI: convert_rpn_to_infix called\n");
    ConversionResult result = rpn_to_infix(rpn);
    printf("FFI: returning expression='%s', error_code=%d\n", result.expression, result.error_code);
    return result;
}