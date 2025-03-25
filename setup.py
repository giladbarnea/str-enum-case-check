from setuptools import setup
import os
import subprocess
import sys
from pathlib import Path

def build_rust_binary():
    """Build the Rust binary before installing the Python package."""
    subprocess.run(["cargo", "build", "--release"], check=True)

# Build the Rust binary if we're installing
if len(sys.argv) > 1 and sys.argv[1] in ["install", "develop"]:
    build_rust_binary()

setup(
    name="str_enum_flake8_plugin",
    version="0.1.0",
    description="A flake8 plugin to check that StrEnum names match their string values",
    author="Your Name",
    author_email="your.email@example.com",
    py_modules=["str_enum_check", "str_enum_flake8"],
    entry_points={
        "console_scripts": [
            "str-enum-check=str_enum_check:main",
        ],
        "flake8.extension": [
            "SE = str_enum_flake8:Plugin",
        ],
    },
    data_files=[
        ("bin", ["target/release/str_enum_flake8_plugin"]),
    ],
    zip_safe=False,
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
    ],
    python_requires=">=3.7",
)