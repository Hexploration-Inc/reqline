# Reqline

**Reqline** is an open-source, local-first, and elegant terminal-based API client built in Rust. It's designed for developers who love the command line and want a fast, keyboard-driven alternative to graphical API clients like Postman or Insomnia.

## Core Philosophy

*   **Local First:** All your collections, history, and environment configurations are stored on your local disk. No cloud sync, no accounts, no strings attached.
*   **TUI Centric:** The user experience is meticulously crafted for the terminal, prioritizing keyboard-driven interaction, speed, and efficiency.
*   **Extensible:** Built with a modular architecture to potentially support plugins, custom themes, and other extensions in the future.
*   **Comprehensive:** Aims to include the most-used features from mainstream API clients, providing a powerful tool for your daily development workflow.

## Proposed Technology Stack

*   **Terminal UI (TUI):** [ratatui](https://ratatui.rs/)
*   **Terminal Backend:** [crossterm](https://github.com/crossterm-rs/crossterm)
*   **HTTP Client:** [reqwest](https://docs.rs/reqwest/latest/reqwest/)
*   **Async Runtime:** [tokio](https://tokio.rs/)
*   **Serialization/Deserialization:** [serde](https://serde.rs/) and [serde_json](https://github.com/serde-rs/json)
*   **Local Storage:** [rusqlite](https://github.com/rusqlite/rusqlite)
*   **Configuration:** [config-rs](https://github.com/mehcode/config-rs) or [toml](https://github.com/toml-rs/toml)

## Feature Roadmap

### Phase 1: The MVP (Minimum Viable Product)

The goal of this phase is to get the core functionality working: making a request and seeing a response.

*   **Single View Layout:**
    *   Pane for defining the request (Method, URL).
    *   Pane for the response (Status Code, Headers, Body).
    *   Basic keybindings to switch between panes and trigger actions.
*   **Request Definition:**
    *   Select HTTP Method (GET, POST, PUT, DELETE, etc.) from a static list.
    *   Input field for the request URL.
    *   A simple table/list editor for adding/removing request headers.
    *   A text area for the request body (raw text only for MVP).
*   **Request Execution:**
    *   Send the defined request using `reqwest`.
    *   Handle loading/pending state in the UI.
*   **Response Viewing:**
    *   Display the HTTP status code and reason phrase (e.g., `200 OK`).
    *   Display response headers in a table.
    *   Display the raw response body in a scrollable text area.

### Phase 2: The "Postman Alternative" (V1.0)

This phase introduces the features that make it a true day-to-day replacement for basic API testing.

*   **Collections & History:**
    *   A new "Collections" pane to display saved requests.
    *   Ability to save the current request (with its URL, method, headers, body) to a collection.
    *   Ability to name and organize requests, potentially in folders.
    *   A "History" pane that automatically logs all sent requests.
    *   Load a request from collections or history back into the main request view.
    *   All data persisted locally using `rusqlite`.
*   **Environments & Variables:**
    *   UI for creating and managing environments (e.g., "Local," "Staging," "Production").
    *   Define key-value variables within each environment (e.g., `base_url`, `auth_token`).
    *   Use variables in the URL, headers, and body with Postman-style syntax (e.g., `{{base_url}}/users`).
    *   An active environment selector in the main UI.
*   **Enhanced UX & Editor:**
    *   **Tabbed Panes:** Use tabs in the request/response sections for `Params`, `Headers`, `Body`, `Cookies`, etc.
    *   **Pretty Printing:** Automatically format and apply syntax highlighting for JSON responses.
    *   **Input Validation:** Basic validation for URL and key-value inputs.
    *   **Vim Keybindings:** Offer an optional Vim mode for navigation.
    *   **Help/Command Palette:** A popup that shows available keybindings and commands.

### Phase 3: The "Fully Featured" Suite (Post-V1.0)

This phase adds advanced features that power users rely on.

*   **Advanced Authentication:**
    *   Built-in helpers for different auth methods (Bearer Token, Basic Auth).
    *   A dedicated flow for OAuth 2.0 (this is a significant undertaking).
*   **Request Body Types:**
    *   Support for `x-www-form-urlencoded` and `multipart/form-data` in addition to raw/JSON.
*   **Testing & Assertions:**
    *   A "Tests" tab where users can write simple assertions against the response.
    *   Example tests: `status is 200`, `header 'Content-Type' contains 'json'`, `json body '.id' exists`.
*   **Import/Export:**
    *   Import existing collections from Postman or OpenAPI/Swagger specifications.
    *   Export Hexplora collections to a shareable format.
*   **GraphQL & gRPC Support:**
    *   Specialized editor for GraphQL queries.
    *   Support for making gRPC requests.
*   **Scripting:**
    *   Support for pre-request and post-request scripts using a lightweight scripting engine like `rhai` or `mlua`.
