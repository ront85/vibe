## Error handling best practices

- **User-Friendly Messages**: Provide clear, actionable error messages to users without exposing technical details or security information
- **Fail Fast and Explicitly**: Validate input and check preconditions early; fail with clear error messages rather than allowing invalid state
- **Specific Exception Types**: Use Rust's Result<T, E> type with custom error types; use specific error types in TypeScript rather than generic ones
- **Centralized Error Handling**: Handle errors at Tauri command boundaries; convert Rust errors to frontend-friendly messages at the interface layer
- **Graceful Degradation**: Design systems to degrade gracefully when non-critical services fail rather than breaking entirely
- **Retry Strategies**: Implement exponential backoff for transient failures in external service calls (model downloads, API calls)
- **Clean Up Resources**: Use Rust's RAII pattern (Drop trait) for resource cleanup; ensure file handles and connections are properly closed
