# 🔥 Calculator Frontend

This is the **GUI frontend** for a Rust-based calculator app. Built with [`egui`](https://github.com/emilk/egui) via [`eframe`](https://github.com/emilk/eframe), it offers a clean, interactive interface that supports multiple calculator modes.

> This frontend is designed to work with the `calculator-backend` crate, which handles all expression parsing and evaluation.

---

## ✨ Features

- **Basic Mode** – Arithmetic operations: `+`, `-`, `*`, `/`, `^`, `%`, `√`, `!`
- **Scientific & Trigonometry Modes** – UI elements for advanced functions
- **History Mode** – Displays past calculations and results
- **Resizable UI Panels** – Custom styles and layouts using `egui`
- **Keyboard Input Support** – Press *Enter* or click the **Enter** button to calculate
- **Clear Button** – Instantly resets your input field
- 🔌 **Backend Integration** – Delegates expression evaluation to the `calculator-backend` crate

---
### 📦 Project Structure
This repository contains both the **backend** and **frontend** components of a Rust-based calculator application. The project demonstrates Rust-C FFI integration for backend calculations and a GUI frontend built with `egui`.

```
calculator-frontend/
├── src/
│   ├── gui.rs         # Main GUI app logic
│   └── main.rs        # App entry point
├── Cargo.toml         # Project metadata and dependencies
```

---

---

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- The `calculator-backend` crate must be available and properly linked

### Installation

1. Clone this repository:
   ```
   git clone https://github.com/your-repo/Rust-Interface-Wrappers-Spring-25.git
   cd Rust-Interface-Wrappers-Spring-25/calculator-frontend
2. Run the application:

    ```
    cargo run 
    ```

---

### 📄 Usage

1. Launch the app
2. Type a math expression (e.g. `2+2`, `5!`, `√9`)
3. Press **Enter** or hit the **Enter** button
4. Switch modes to explore different function panels (WIP)

---

### 🛠️ Under the Hood

- The main app is defined in [`lib.rs`](src/lib.rs), implementing the `eframe::App` trait
- Expression processing is delegated to the backend via `calculate_expression(&self.input_value)`
- The main entry point is in [`main.rs`](src/bin/main.rs), where the window is sized and initialized

---

### 📌 Dependencies

```toml
[dependencies]
eframe = "0.31.1"
calculator-backend = "0.1.12"
```

---

### 🧭 To-Do

- Improve error handling for backend responses
- Add support for keyboard shortcuts and history navigation
- Enhance UI styling and responsiveness
