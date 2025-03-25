import enum
from enum import StrEnum, auto

# This should trigger an error
class Colors(StrEnum):
    red = "RED"  # Error: name is lowercase but value is uppercase
    BLUE = "BLUE"  # Valid: name and value casing match
    green = auto()  # Valid: using auto()

# This should also trigger an error
class Status(enum.StrEnum):
    success = "SUCCESS"  # Error: name is lowercase but value is uppercase
    ERROR = "ERROR"  # Valid: name and value casing match