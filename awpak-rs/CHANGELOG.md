# Changelog

All notable changes to this project will be documented in this file.

## [0.0.2]
### Added
- **Configuration File Support**: Added the `#[config_file]` macro to load structured configuration from JSON files.
  - Supports defining typed structs with `#[derive(Deserialize)]`.
  - Allows using an empty struct to store the configuration as a `serde_json::Value`.
  - Supports environment variable substitution in configuration values.
  - Uses the `AWPAK_RS_HOME` environment variable to determine the application's home directory.

### Changed
- **Renamed `BodyData` to `RequestBody`**:
  - The struct `BodyData` has been renamed to `RequestBody`.
  - It has been moved from `awpak_rs::body::body::BodyData` to `awpak_rs::io::request::request_body::RequestBody`.
  - Update imports accordingly:
    ```rust
    use awpak_rs::io::request::request_body::RequestBody;
    ```

## [0.0.1] - Initial Release
- Basic routing macros: `#[get]`, `#[post]`, `#[put]`, `#[delete]`, etc.
- Query parameter extraction (`#[query_param]`, `#[query_params]`).
- Path variable extraction (`#[path_variable]`).
- Middleware support (`#[middleware]`).
- Request/response headers and cookies handling.
- Context sharing between middleware and endpoints.
- File upload support for `multipart/form-data`.