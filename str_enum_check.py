#!/usr/bin/env python3
"""
A Python wrapper for the str_enum_flake8_plugin Rust binary.
This script makes it easier to integrate the tool with Python workflows.
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path

def main():
    parser = argparse.ArgumentParser(
        description="Validate that StrEnum subclasses have consistent casing"
    )
    parser.add_argument(
        "paths", nargs="+", help="Python files or directories to check"
    )
    args = parser.parse_args()

    # Determine the path to the Rust binary
    script_dir = Path(__file__).parent
    binary_path = script_dir / "target" / "release" / "str_enum_flake8_plugin"
    if not binary_path.exists():
        binary_path = script_dir / "target" / "debug" / "str_enum_flake8_plugin"
        if not binary_path.exists():
            print("Error: Cannot find the str_enum_flake8_plugin binary.")
            print("Please run 'cargo build' first.")
            sys.exit(1)

    # Check each path
    errors_found = False
    for path in args.paths:
        if not os.path.exists(path):
            print(f"Error: Path '{path}' does not exist.")
            continue

        try:
            result = subprocess.run(
                [str(binary_path), "--path", path],
                capture_output=True,
                text=True,
                check=False
            )
            
            if result.returncode != 0:
                # If there was output and a non-zero return code, errors were found
                if result.stdout:
                    print(result.stdout, end="")
                elif result.stderr and "Error:" not in result.stderr:
                    print(result.stderr, end="")
                errors_found = True
            elif len(result.stdout.strip()) > 0:
                print(result.stdout, end="")
                
        except subprocess.CalledProcessError as e:
            print(f"Error running check on {path}: {e}")
            errors_found = True
    
    return 1 if errors_found else 0

if __name__ == "__main__":
    sys.exit(main())