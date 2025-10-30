## General development conventions

- **Consistent Project Structure**: Maintain Cargo workspace structure with `core/` (library) and `desktop/src-tauri/` (app); organize React components in `desktop/src/`
- **Clear Documentation**: Maintain up-to-date README and `docs/` directory with build instructions, debugging, and model guides
- **Version Control Best Practices**: Use clear commit messages, feature branches, and meaningful pull/merge requests with descriptions
- **Environment Configuration**: Use environment variables for configuration; never commit secrets or API keys to version control; API keys for Claude/Ollama should be user-provided
- **Dependency Management**: Keep dependencies up-to-date and minimal; use Cargo features for optional GPU backends (cuda, vulkan, metal, coreml, rocm)
- **Code Review Process**: Establish a consistent code review process with clear expectations for reviewers and authors
- **Testing Requirements**: Run `cargo test -p vibe_core --release -- --nocapture` for core library tests before merging
- **Feature Flags**: Use Cargo features for optional functionality (GPU backends, server mode) rather than runtime configuration where possible
- **Changelog Maintenance**: Update `docs/changelog.md` to track significant changes and improvements
