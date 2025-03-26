#!/usr/bin/env python3
"""
Python wrapper script for the strenum_case binary.
This script will find and run the appropriate binary for the current system.
"""

import os
import sys
import subprocess
import platform

def find_binary():
    """
    Find the strenum_case binary.
    First, check if it's in the same directory as this script.
    Then, check if it's in the ./target/release directory (for development).
    """
    script_dir = os.path.dirname(os.path.abspath(__file__))

    # Define possible binary locations and names
    binary_name = "strenum_case"
    if platform.system() == "Windows":
        binary_name += ".exe"

    binary_locations = [
        os.path.join(script_dir, binary_name),
        os.path.join(script_dir, "target", "release", binary_name),
        os.path.join(script_dir, "target", "debug", binary_name),
    ]

    # Check if the binary exists in any of the locations
    for location in binary_locations:
        if os.path.isfile(location):
            return location

    raise FileNotFoundError(
        f"Could not find the strenum_case binary. "
        f"Please make sure it's installed correctly.\n"
        f"You can build it with 'cargo build --release'."
    )


def main():
    """Run the strenum_case binary with the given arguments."""
    try:
        binary = find_binary()
        args = ["--path"] + sys.argv[1:] if len(sys.argv) > 1 else ["--help"]
        process = subprocess.run([binary] + args, check=True)
        sys.exit(process.returncode)
    except FileNotFoundError as e:
        print(str(e), file=sys.stderr)
        sys.exit(1)
    except subprocess.CalledProcessError as e:
        sys.exit(e.returncode)

if __name__ == "__main__":
    main()