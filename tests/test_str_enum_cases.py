import ast
import enum
from enum import StrEnum, auto, Enum

########################################
# Example cases from README categories #
########################################

# SKIPPED CASES


# Skipped: name and value are different strings
class SkippedDifferentStrings(StrEnum):
    a = "Hello"


# Skipped: only StrEnum subclasses are checked
class SkippedNotStrEnum(Enum):
    a = "a"
    b = "b"


# INVALID CASES


# Invalid: name and value have different case
class InvalidDifferentCase(StrEnum):
    a = "A"


# Invalid: name and value have different case (reverse)
class InvalidDifferentCaseReverse(StrEnum):
    A = "a"


# Invalid: member names have inconsistent case
class InvalidInconsistentNames(StrEnum):
    a = "a"
    B = "B"


# Invalid: member value is auto() and name is uppercase
class InvalidUpperAuto(StrEnum):
    A = auto()


# Invalid: uppercase with qualified auto()
class InvalidUpperQualifiedAuto(StrEnum):
    B = enum.auto()


# Invalid: member names have inconsistent case with auto()
class InvalidMixedWithAuto(StrEnum):
    A = "A"
    b = auto()


# Invalid: complex expressions
ello = "ello"
orld = "orld"


class InvalidComplexValues(StrEnum):
    hello = "H" + ello  # Inconsistent case
    World = f"w{orld}"


# VALID CASES


# Valid: name and value case match (lowercase)
class ValidLowercase(StrEnum):
    a = "a"
    b = "b"


# Valid: name and value case match (uppercase)
class ValidUppercase(StrEnum):
    A = "A"
    B = "B"


# Valid: member value is auto() and name is lowercase
class ValidLowercaseAuto(StrEnum):
    a = auto()
    b = enum.auto()


# Valid: complex expressions
ello = "ello"
arth = "arth"


class ValidComplexValues(StrEnum):
    hello = "h" + ello
    world = enum.auto()
    earth = f"e{arth}"


# ADVANCED TEST CASES


# Test with nested enum
class Outer:
    class NestedEnum(enum.StrEnum):
        a = "A"  # Invalid: mismatched case


# Test case with multiple inheritance
code_multi_inherit = """
from enum import StrEnum

class BaseClass:
    pass

class MultiInherit(BaseClass, StrEnum):
    VALID = "VALID"
    invalid = "INVALID"  # Should be flagged
"""


def test_str_enum_cases():
    """Test all the StrEnum case validation scenarios."""
    from str_enum_case_check.str_enum_case_check_flake8 import StrEnumVisitor

    # Test all cases defined within this file
    with open(__file__, "r") as f:
        tree = ast.parse(f.read())

    visitor = StrEnumVisitor()
    visitor.visit(tree)

    print(f"Found {len(visitor.errors)} errors:", visitor.errors)

    # Expected invalid cases
    invalid_classes = [
        "InvalidDifferentCase",
        "InvalidDifferentCaseReverse",
        "InvalidInconsistentNames",
        "InvalidUpperAuto",
        "InvalidMixedWithAuto",
        "NestedEnum",
    ]

    # Verify all invalid cases are detected
    for class_name in invalid_classes:
        assert any(
            class_name in err[2] for err in visitor.errors
        ), f"Failed to detect invalid class: {class_name}"

    # Verify valid/skipped cases are NOT flagged
    skipped_classes = [
        "SkippedDifferentStrings",
        "SkippedNotStrEnum",
        "SkippedComplexValues",
        "ValidLowercase",
        "ValidUppercase",
        "ValidLowercaseAuto",
    ]

    for class_name in skipped_classes:
        assert not any(
            class_name in err[2] for err in visitor.errors
        ), f"Incorrectly flagged valid/skipped class: {class_name}"

    # Test multiple inheritance
    tree1 = ast.parse(code_multi_inherit)
    visitor1 = StrEnumVisitor()
    visitor1.visit(tree1)
    assert any(
        "MultiInherit" in err[2] for err in visitor1.errors
    ), "Failed to detect invalid MultiInherit class"

    print("All test cases passed!")


if __name__ == "__main__":
    test_str_enum_cases()
