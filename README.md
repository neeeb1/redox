# **Redox: Your Personal TUI Journaling Companion**

### 🚧 This project is a work in progress! Some features may not be implemented yet and stability is not guaruanteed 🚧


## Overview
Redox is an intuitive Terminal User Interface (TUI) application built with Rust, designed to streamline your daily journaling experience. It provides a structured way to capture your thoughts, gratitude, media consumption, and tasks, organizing them into a clean Markdown file for easy review and storage. Leveraging the power of `ratatui` for its interactive interface and `tera` for flexible output templating, Redox offers a lightweight yet powerful solution for digital journaling directly from your terminal.

## Features
-   **Interactive TUI:** A responsive and easy-to-navigate terminal interface for a focused journaling experience.
-   **Customizable Prompts:** Define your own set of daily prompts via a simple JSON configuration file (`.redox.json`).
-   **Markdown Output:** Automatically compile your journal entries into a neatly formatted Markdown file, perfect for archival or integration with other tools.
-   **Offline First:** All journaling happens locally, ensuring privacy and accessibility without an internet connection.

## Getting Started

### Installation
To get started with Redox, you'll need Rust and Cargo installed. If you don't have them, please follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/neeeb1/redox.git
    cd redox
    ```

2.  **Build the Project**:
    ```bash
    cargo build --release
    ```
    This command compiles the application and places the executable in `target/release/`.

3.  **Run the Application**:
    ```bash
    cargo run --release
    ```
    Alternatively, you can run the compiled executable directly:
    ```bash
    ./target/release/redox
    ```

### Environment Variables
Redox manages its configuration through a `.redox.json` file, typically located in your home directory. This file defines the journal prompts available to you.

**`.redox.json` Example:**
If the `.redox.json` file does not exist in your home directory, Redox will create it with the following default structure:
```json
{
    "prompts": [
        {
            "name": "Today's Thoughts",
            "prompt": "What are you thinking about today? Any recurring thoughts that you can't get out of your head?"
        },
        {
            "name": "Gratitude is Rad-itude",
            "prompt": "What are you thankful for today? Try to name 2 or 3 things."
        },
        {
            "name": "My Media Diet",
            "prompt": "What are you consuming lately? Games, movies, music, books... anything!"
        },
        {
            "name": "Today's Top Tasks",
            "prompt": "Name 3 tasks that are essential for today.\nIf you did these, you've done the bare minimum for a successful day!"
        }
    ]
}
```
You can customize these prompts by editing the `~/.redox.json` file. Each prompt requires a `name` (for display in the TUI) and a `prompt` (the question itself).

## Usage
Once you run Redox, you will be greeted by the TUI.

1.  **Prompt Selection Mode:**
    *   Use the `Up` and `Down` arrow keys to navigate through the list of available journal prompts.
    *   Press `Enter` to toggle the selection of a prompt.
    *   Once you've selected all the prompts you wish to answer for the day, navigate to the "Continue" option at the bottom of the list and press `Enter`.
    *   To quit the application at any time, press `q`.

2.  **Entry Mode:**
    *   After continuing from the selection screen, you will enter entry mode, where you will be presented with each selected prompt one by one.
    *   Type your journal entry directly into the text area. The application supports multi-line input.
    *   When you are satisfied with your response for a prompt, press `Enter` to submit your entry and move to the next prompt.
    *   A progress bar at the bottom will show you how many entries you have completed versus the total selected.
    *   If you need to exit the application during entry mode, press `Esc`.

3.  **Wrap-Up Mode:**
    *   After completing all selected entries, you will enter Wrap-Up mode.
    *   Here, you can review all your entries for the day.
    *   Press `Enter` to finalize your journal and save it to a Markdown file.
    *   To quit the application without saving, press `q`.

**Output File:**
Your finalized journal entries will be saved to `output/daily.md` within the project directory. Each entry will be formatted according to the `templates/daily.md` structure, including the date and each prompt with its respective answer.

## Technologies Used

| Technology         | Description                                                      | Link                                                 |
| :----------------- | :--------------------------------------------------------------- | :--------------------------------------------------- |
| **Rust**           | Primary programming language.                                    | [rust-lang.org](https://www.rust-lang.org/)          |
| **Ratatui**        | Framework for building TUI applications.                         | [ratatui.rs](https://ratatui.rs/)                    |
| **Crossterm**      | Cross-platform terminal manipulation library.                    | [docs.rs/crossterm](https://docs.rs/crossterm)       |
| **Tera**           | Template engine for rendering Markdown output.                   | [tera.netlify.app](https://tera.netlify.app/)        |
| **Serde**          | Serialization/deserialization framework for JSON configuration.  | [serde.rs](https://serde.rs/)                        |
| **Config**         | Configuration management library.                                | [docs.rs/config](https://docs.rs/config)             |
| **Chrono**         | Date and time library for timestamping entries.                  | [docs.rs/chrono](https://docs.rs/chrono)             |
| **Dirs**           | Utility to find common user directories (e.g., home directory).  | [docs.rs/dirs](https://docs.rs/dirs)                 |
| **Ratatui-TextArea** | A `ratatui` widget for multi-line text input.                    | [docs.rs/ratatui-textarea](https://docs.rs/ratatui-textarea) |

## License
This project is licensed under the MIT License. See the `LICENSE` file in the repository for full details.

---
[![Rust Programming Language](https://img.shields.io/badge/Rust-red?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/neeeb1/redox/graphs/commit-activity)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)