import ast
import enum
from enum import StrEnum, auto, Enum

# Case 1: Valid class with mixed auto() and string members
class MixedValid(StrEnum):
    FOO = "FOO"  # Valid: matching case
    BAR = auto()  # Valid: using auto()
    BAZ = enum.auto()  # Valid: using qualified auto()

# Case 2: Non-StrEnum class should be ignored
class NotStrEnum(Enum):
    a = "A"  # Should be ignored

# Case 3: With from_dict class method
class WithClassMethod(StrEnum):
    a = "A"  # Invalid: mismatched case
    
    @classmethod
    def from_dict(cls, data):
        return cls(data["value"])

# Case 4: Nested enum
class Outer:
    class NestedEnum(enum.StrEnum):
        a = "A"  # Invalid: mismatched case

# Case 5: Complex values
class ComplexValues(StrEnum):
    A = "A" + "A"  # Should be ignored - not a simple string literal
    B = f"B"  # Should be ignored - not a simple string literal

def test_edge_cases():
    from str_enum_plugin import StrEnumVisitor
    
    with open(__file__, 'r') as f:
        tree = ast.parse(f.read())
    
    visitor = StrEnumVisitor()
    visitor.visit(tree)
    
    print("Found errors:", visitor.errors)
    
    # Should find 2 invalid members
    assert len(visitor.errors) == 2
    
    # Check that errors are for the right members
    assert any("WithClassMethod" in err[0] and "a" in err[0] for err in visitor.errors)
    assert any("NestedEnum" in err[0] and "a" in err[0] for err in visitor.errors)
    
    # Verify all these are NOT flagged
    for class_name in ["NotStrEnum", "ComplexValues"]:
        assert not any(class_name in err[0] for err in visitor.errors)

if __name__ == "__main__":
    test_edge_cases()