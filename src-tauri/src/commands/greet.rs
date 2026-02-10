//! Example greet command demonstrating the Tauri IPC pattern.

/// Returns a greeting message for the given name.
///
/// # Errors
///
/// Returns `AppError::Validation` if `name` is empty.
///
/// # Frontend Usage
///
/// ```typescript
/// const message = await invoke<string>("greet", { name: "World" });
/// ```
#[tauri::command]
pub fn greet(name: &str) -> Result<String, crate::error::AppError> {
    if name.is_empty() {
        return Err(crate::error::AppError::Validation(
            "Name cannot be empty".to_string(),
        ));
    }
    Ok(format!("Hello, {}! Welcome to your Tauri app.", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_returns_greeting() {
        let result = greet("World").unwrap();
        assert!(result.contains("World"));
    }

    #[test]
    fn test_greet_empty_name_returns_validation_error() {
        let result = greet("");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::error::AppError::Validation(_)
        ));
    }
}
