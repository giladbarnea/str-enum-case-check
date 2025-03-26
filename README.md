# str_enum_case_check

A flake8 plugin and standalone tool to validate that Python `StrEnum` subclasses have consistent casing between the member names and their string values.

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
```

This would also be valid:

```python
class A(enum.StrEnum):
    a = auto()       # Valid: using auto()
    b = enum.auto()  # Valid: using qualified auto()
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
./str_enum_case_check.py path/to/code
```

Or directly use the Rust binary:

```
./target/release/str_enum_case_check --path path/to/code
```

## Usage

### Basic Usage

```
str_enum_case_check --path /path/to/project
```

### Excluding Directories

You can exclude specific directories using the `--exclude` option with a comma-separated list:

```
str_enum_case_check --path /path/to/project --exclude=test,migrations,frontend
```

By default, the tool already skips:
- Directories starting with `.` (e.g., `.venv`, `.git`)
- Directories starting with `_` (e.g., `__pycache__`)
- Common build directories (`build`, `dist`)
- Python virtual environments (`venv`, `env`)
- Directories ending with `.egg-info`
- Non-Python files (only `.py` files are processed, not `.pyc`, `.pyi`, etc.)

### Help

```
str_enum_case_check --help
```

## Performance

The tool is optimized for performance with parallel file processing and efficient string handling. On a medium-sized codebase, it typically runs in under 0.1~0.2 seconds.

## Error Codes

- **SE001**: StrEnum member has inconsistent casing between name and value

## Full Spec

`StrEnum` subclasses whose members' names and values are different strings, regardless of case, are skipped.

A `StrEnum` subclass is invalid if it matches any of the following patterns:

1. Member name and value have different case.
2. Member names have inconsistent case within the same `StrEnum` subclass.
3. Member value is `auto()` and the member name is uppercase.

### Examples

#### Skipped
```python
# Skipped: name and value are different strings
class A(StrEnum):
    a = "Hello"

# Skipped: only `StrEnum` subclasses are checked
class A(Enum):
    a = "a"
```

#### Invalid
```python
# Invalid: name and value have different case
class A(StrEnum):
    a = "A"
```

```python
# Invalid: name and value have different case
class A(StrEnum):
    A = "a"
```

```python
# Invalid: member names have inconsistent case
class A(StrEnum):
    a = "a"
    B = "B"
```

```python
# Invalid: member value is auto() and name is uppercase
class A(StrEnum):
    A = auto()
```

```python
# Invalid: member names have inconsistent case
class A(StrEnum):
    A = "A"
    b = auto()
```

#### Valid
```python
# Valid: name and value case match
class A(StrEnum):
    a = "a"
```

```python
# Valid: name and value case match
class A(StrEnum):
    A = "A"
```

```python
# Valid: member value is auto() and name is lowercase
class A(StrEnum):
    a = auto()
```

```python
class BaseClass: ...

ello = "ello"
arth = "arth"

class A(BaseClass, enum.StrEnum):
    hello = "h" + ello
    world = enum.auto()
    earth = f"e{arth}
```


## License

MIT