# Installing flake8-str-enum

## Installation Methods

### Using pip

```bash
pip install flake8-str-enum
```

### From Source

1. Clone the repository:
```bash
git clone https://github.com/username/flake8-str-enum.git
cd flake8-str-enum
```

2. Install in development mode:
```bash
pip install -e .
```

## Usage

After installation, flake8 will automatically use this plugin when you run flake8:

```bash
flake8 your_project/
```

To run only the checks from this plugin:

```bash
flake8 --ignore=E,F,W --select=SE your_project/
```

## Error Codes

- `SE001`: StrEnum member has inconsistent casing between name and value