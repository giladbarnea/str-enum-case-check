# str-enum-case-check Commands and Guidelines

## Commands
- Build Rust binary: `cargo build --release`
- Quick development iteration: `cargo run -- --path path/to/code` (builds dev version on the fly)
- Run tests: `cargo build && uv run pytest`
- Install dependencies: `uv sync`
- Install new package: `uv add [--dev] PACKAGE`

## Project Structure
- Rust implementation with Python wrapper: Core functionality in Rust for performance with a thin Python wrapper for ease of use
- Error code: SE001 - StrEnum member has inconsistent casing
- Exclude patterns are configurable with `--exclude` flag

## Development
- Performance is critical: has to be less than 200ms execution even on large codebases
- Write simple, idiomatic Rust code that's easy to maintain
- Follow Rust best practices
- Run 'cargo build; uv run pytest' frequently between changes

## Claude Instructions
- When fixing or writing, make minimal, surgical changes
- Follow the user's instructions: not more, not less
