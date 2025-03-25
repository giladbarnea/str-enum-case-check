import enum
from enum import StrEnum, auto

# Invalid: name casing doesn't match value
class Invalid(StrEnum):
    a = "A"
    b = "B"

# Valid: using auto()
class Valid(StrEnum):
    A = auto()
    B = enum.auto()

# Invalid: using qualified name
class QualifiedInvalid(enum.StrEnum):
    a = "A"

# Valid: same casing
class ValidSameCasing(StrEnum):
    UPPER = "UPPER"
    lower = "lower"
    
# Mixed: some valid, some invalid
class Mixed(StrEnum):
    VALID = "VALID"
    invalid = "INVALID"
    also_valid = auto()