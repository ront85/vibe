## Coding style best practices

- **Consistent Naming Conventions**: Follow Rust conventions (snake_case for functions/variables, PascalCase for types) and TypeScript conventions (camelCase for functions/variables, PascalCase for components/types)
- **Automated Formatting**: Use `cargo fmt` for Rust and Prettier for TypeScript/React; format on save enabled in VSCode
- **Meaningful Names**: Choose descriptive names that reveal intent; avoid abbreviations and single-letter variables except in narrow contexts
- **Small, Focused Functions**: Keep functions small and focused on a single task for better readability and testability
- **Consistent Indentation**: Use consistent indentation (spaces or tabs) and configure your editor/linter to enforce it
- **Remove Dead Code**: Delete unused code, commented-out blocks, and imports rather than leaving them as clutter; run `cargo clippy` to catch unused code
- **Backward compatibility only when required:** Unless specifically instructed otherwise, assume you do not need to write additional code logic to handle backward compatibility.
- **DRY Principle**: Avoid duplication by extracting common logic into reusable functions or modules (core library for shared transcription logic)
