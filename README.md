# str_enum_flake8_plugin

A flake8 plugin and standalone tool to validate that Python StrEnum subclasses have consistent casing between the member names and their string values.

## Description

This tool scans Python files for StrEnum subclasses and flags instances where the case of a member name does not match the case of its string value.

For example, this would be flagged as invalid:

```python
class A(StrEnum):
    a = "A"  # Error: name is lowercase but value is uppercase
```

This would be valid:

```python
class A(StrEnum):
    A = "A"  # Valid: name and value casing match
    b = auto()  # Valid: using auto()
    c = enum.auto()  # Valid: using qualified auto()
```

The plugin correctly handles both direct inheritance from `StrEnum` and from `enum.StrEnum`.

## Installation

### Python-only installation (using flake8)

```
pip install .
```

Then you can use it with flake8:

```
flake8 --select=SE your_file.py
```

### Using the standalone tool

First, build the Rust binary:

```
cargo build --release
```

Then you can use the Python wrapper script:

```
./str_enum_check.py path/to/code
```

Or directly use the Rust binary:

```
./target/release/str_enum_flake8_plugin --path path/to/code
```

## How it works

The plugin uses AST parsing to find StrEnum classes and their members. For each member that has a string literal value (not auto()), it checks if the member name and value have the same case.

## Error Codes

- **SE001**: StrEnum member has inconsistent casing between name and value

## License

MIT