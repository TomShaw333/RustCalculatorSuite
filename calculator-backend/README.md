# Rust-Interface-Wrappers-Spring-25

Rust Foreign Function Interface Wrapper Crate	 
RI: Daniel Dusharm ddusharm@asrcfederal.com
 
External bindings make the (computer) world go round. For example, Python AI/ML projects that use performance-critical or efficient libraries do so by leveraging foreign function interfaces (FFI) to call native code. This project demonstrates the development of a wrapper crate in Rust using `std::ffi` to expose the functionality of an external C library while ensuring type safety. Students will gain hands-on experience with Rust fundamentals such as memory safety, ownership, and FFI integration.

## Project Overview

This project implements a calculator library that demonstrates Rust-C FFI integration. It consists of a C library for core calculations and a Rust wrapper providing a safe interface through a command-line interface (CLI).


### Key Features

- Infix to RPN (Reverse Polish Notation) conversion
- Mathematical expression evaluation
- RESTful API interface
- Memory-safe FFI implementation
- Robust error handling
- Support for complex mathematical expressions
- History saves all user input with answers or errors 

## Installation

### Prerequisites
- Rust (latest stable)
- C compiler (gcc/clang)
- Cargo

## Usage

### Running the Application

To start the calculator application, run the following command:

cargo run --bin main

## Project Alignment

This implementation aligns with the project goals in several ways:

1. **FFI Integration**
   - Uses `std::ffi` for C interop
   - Handles C strings and data types
   - Manages memory across language boundaries

2. **Memory Safety**
   - Safe wrapper around unsafe C code
   - Proper memory management
   - Rust ownership rules enforcement

3. **Error Handling**
   - Comprehensive error types
   - Safe error propagation
   - User-friendly error messages

4. **Type Safety**
   - Strong type checking
   - Safe type conversions
   - Rust type system utilization

## Technical Details

### Memory Management

The project handles memory safety through:
- RAII principles in Rust
- Careful C memory allocation/deallocation
- Smart pointer usage
- Automatic cleanup on scope exit

### Error Handling

Error types include:
- Division by zero
- Invalid operators
- Stack underflow
- Memory errors
- Undefined variables
- Stack maximum
- Expretion Length Maximum
- Factorial errors

### Type Conversions

Handles conversions between:
- Rust strings ↔ C strings
- Rust numbers ↔ C numbers
- Complex data structures
- Error codes

## Testing

### Run all tests
cargo test

### Run a specific test
cargo test --test <test_name>

## Limitations and Future Work

Current limitations:

- Basic operator set

Planned improvements:
- Scientific notation parsing for negative numbers
- Dynamic memory allocation
- Additional mathematical functions
- Variable support
- Function definitions

### Additional Enhancements

1. **Additional Mathematical Functions**
   - Trigonometric functions (sin, cos, tan)
   - Logarithmic functions (ln, log)
   - Constants (pi, e)

2. **Expression Optimization**
   - Constant folding
   - Expression simplification
   - Performance improvements

### Testing Requirements

1. **Test Coverage**
   - Add unit tests for negative numbers
   - Add tests for Unicode handling
   - Add scientific notation tests
   - Add stress tests for large expressions
   - Add history testing with rezults and errors
   - Add factorial testing with rezults and errors 

### Documentation Updates

1. **API Documentation**
   - Document all supported operators
   - Add examples for all edge cases
   - Update error messages

### Performance Improvements

1. **Optimization Opportunities**
   - Memory allocation optimization
   - Parser performance improvements
   - Error handling efficiency
   - String handling optimization

## License

This project is part of academic coursework and is subject to course guidelines.

## Acknowledgments

- Daniel Dusharm (Project Advisor)
- ASRC Federal
- Dr. Neil Toporski (Course Instructor)
- Project Team Members

