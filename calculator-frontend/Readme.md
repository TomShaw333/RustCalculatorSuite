## 🔥 Calculator Frontend

This is the **GUI frontend** for a Rust-based calculator app. Built with [`egui`](https://github.com/emilk/egui) via [`eframe`](https://github.com/emilk/eframe), it offers a clean, interactive interface that supports multiple calculator modes.

> This crate is designed to work with a separate `calculator-backend` crate, responsible for parsing and evaluating expressions.

---

### ✨ Features

- **Basic Mode** – Arithmetic operations: `+`, `-`, `*`, `/`, `^`, `%`, `√`, `!`
- **Scientific & Trigonometry Modes** – UI elements in place for future expansion
- **Resizable UI panels** – Custom styles and layouts using `egui`
- **Keyboard input support** – Press *Enter* or click the **Enter** button to calculate
- **Clear button** – Instantly resets your input field
- 🔌 **Connected to a backend** that handles all expression evaluation

---

### 📦 Project Structure

```
calculator-frontend/
├── src/
│   ├── gui.rs         # Main GUI app logic
│   └── main.rs        # App entry point
├── Cargo.toml         # Project metadata and dependencies
```

---

### 🚀 Getting Started

Make sure you have Rust installed.

1. Clone this repository
2. Ensure the [`calculator-backend`](https://crates.io/crates/calculator-backend) crate is available and properly linked
3. Run the application:

```bash
cargo run main
```

---

### 📄 Usage

1. Launch the app
2. Type a math expression (e.g. `2+2`, `5!`, `√9`)
3. Press **Enter** or hit the **Enter** button
4. Switch modes to explore different function panels (WIP)

---

### 🛠️ Under the Hood

- The main app is defined in [`gui.rs`](src/gui.rs), implementing the `eframe::App` trait
- Expression processing is delegated to the backend via `calculate_expression(&self.input_value)`
- The main entry point is in [`main.rs`](src/bin/main.rs), where the window is sized and initialized

---

### 📌 Dependencies

```toml
[dependencies]
eframe = "0.31.1"
calculator-backend = "0.1.2"
```

---

### 🧭 To-Do

- Implement logic for Scientific and Trigonometric functions
- Handle errors from backend more gracefully
- Add support for keyboard shortcuts and history
