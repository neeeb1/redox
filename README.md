# **Redox: A Minimalist Journaling TUI**

### 🚧 This project is a work in progress! 🚧

## Overview
Redox is an interactive Terminal User Interface (TUI) application crafted in Rust, providing a focused environment for structured journaling. Users can select from a predefined set of prompts to record their thoughts, reflections, and daily activities directly within the terminal, leveraging the power of `ratatui` for a clean and efficient experience.

## Features
-   **Interactive Prompt Selection**: Choose multiple prompts for your journaling session using intuitive keyboard navigation.
-   **Configurable Prompts**: Customize and expand your journaling experience by defining prompts in a simple JSON configuration file.
-   **Clean User Interface**: A distraction-free terminal interface powered by `ratatui` for an immersive writing experience.
-   **Intuitive Navigation**: Easily move between prompts and selections with standard keyboard commands.
-   **High Performance**: Built with Rust, ensuring a responsive and efficient application.

## Getting Started

### Installation
To get Redox up and running on your local machine, follow these steps:

1.  **Prerequisites**:
    *   Ensure you have the [Rust toolchain](https://rustup.rs/) installed. This includes `cargo`, Rust's package manager and build tool.

2.  **Clone the Repository**:
    ```bash
    git clone https://github.com/neeeb1/redox.git
    ```

3.  **Navigate to Project Directory**:
    ```bash
    cd redox
    ```

4.  **Build the Project**:
    ```bash
    cargo run
    ```
    This command compiles the application and creates an optimized executable in the `target/release/` directory.

### Environment Variables
No specific environment variables are required to run Redox. All configurations are handled via the `.redox.json` file.

## Usage

### Configuration File
Redox loads its journaling prompts from a `.redox.json` file located in your home directory (`~/.redox.json`). If this file does not exist, an empty one will be created, which will cause the application to panic upon startup as it expects a valid JSON structure. Please create this file manually if it doesn't exist and populate it with your desired prompts, following the example below:

**`~/.redox.json` Example:**
```json
{
  "prompts": [
    {
      "name": "Today's Thoughts",
      "prompt": "What are you thinking about today? Any recurring thoughts that you can't get out of your head?"
    },
    {
      "name": "Graditude is Rad-itude",
      "prompt": "What are you thankful for today? Try to name 2 or 3 things."
    },
    {
      "name": "My Media Diet",
      "prompt": "What are you consuming lately? Games, movies, music, books... anything!"
    },
    {
      "name": "Today's Top Tasks",
      "prompt": "name 3 tasks that are essential for today.\nIf you did these, you've done the bare minimum for a successful day!"
    }
  ]
}
```

### Running the Application
After building, you can run Redox using Cargo or directly executing the compiled binary:

```bash
cargo run --release
```
Alternatively, execute the compiled binary:
```bash
./target/release/redox
```

### Navigating the TUI

#### Selection Mode
Upon launching, Redox presents a list of available journaling prompts.

-   **Navigate**: Use the `Up` and `Down` arrow keys to move through the list of prompts.
-   **Toggle Selection**: Press `Enter` to select or deselect a prompt. A `[X]` will appear next to selected prompts.
-   **Continue**: Once you have selected your desired prompts, navigate to the "Continue" option and press `Enter` to proceed to the entry mode.
-   **Quit**: Press `q` to exit the application from selection mode.

#### Entry Mode
After selecting "Continue", you will enter the journaling interface for the first selected prompt.

-   **Type Entry**: A text area will be displayed where you can type your journal entry in response to the prompt.
-   **Exit**: To exit the application from entry mode, press `Esc`.

## Technologies Used

| Technology         | Description                                        | Link                                                                      |
| :----------------- | :------------------------------------------------- | :------------------------------------------------------------------------ |
| **Rust**           | Primary programming language.                      | [Rust-Lang](https://www.rust-lang.org/)                                   |
| **Ratatui**        | Framework for building rich Terminal User Interfaces (TUIs). | [Ratatui GitHub](https://github.com/ratatui-org/ratatui)                |
| **Crossterm**      | Cross-platform terminal manipulation library for Rust. | [Crossterm GitHub](https://github.com/crossterm-rs/crossterm)             |
| **ratatui-textarea** | A `ratatui` widget for multi-line text input.      | [ratatui-textarea GitHub](https://github.com/7sDream/ratatui-textarea)   |
| **config**         | Hierarchical configuration solution for Rust.      | [config-rs GitHub](https://github.com/mehcode/config-rs)                  |
| **serde**          | A generic serialization/deserialization framework. | [serde.rs](https://serde.rs/)                                             |
| **dirs**           | A Rust library for getting standard directories.   | [dirs-rs GitHub](https://github.com/soc/dirs-rs)                          |

## License
This project is licensed under the MIT License.

[![Rust](https://img.shields.io/badge/Rust-red?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/Ratatui-blue?style=flat)](https://github.com/ratatui-org/ratatui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)