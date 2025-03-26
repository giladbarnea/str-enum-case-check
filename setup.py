import subprocess
import sys

from setuptools import setup


def build_rust_binary():
    """Build the Rust binary before installing the Python package."""
    subprocess.run(["cargo", "build", "--release"], check=True)


# Build the Rust binary if we're installing
if len(sys.argv) > 1 and sys.argv[1] in ["install", "develop"]:
    build_rust_binary()

setup(
    name="str_enum_case_check",
    version="0.1.0",
    description="A flake8 plugin to check that StrEnum names match their string values",
    author="Gilad Barnea",
    author_email="giladbrn@gmail.com",
    url="https://github.com/giladbarnea/str-enum-case-check",
    py_modules=["MISSING FLAKE8 PLUGIN PYTHON FILE", "OTHER DEPENDENCIES"],
    install_requires=[
        "flake8>=3.0.0",
    ],
    entry_points={
        "flake8.extension": [
            "SE001 = strenum_plugin:StrEnumPlugin",
        ],
    },
    classifiers=[
        "Framework :: Flake8",
        "Environment :: Console",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Software Development :: Quality Assurance",
    ],
    python_requires=">=3.10",
    scripts=["str_enum_case_check.py"],
    # Include the Rust binary in the package
    package_data={
        "": ["target/release/str_enum_case_check"],
    },
    # Make sure the package can still be installed without the Rust binary
    # (users can use pip to install and then build the Rust binary separately)
    zip_safe=False,
)
