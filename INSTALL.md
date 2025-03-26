# Installing flake8-str-enum-case-check

TODO

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
