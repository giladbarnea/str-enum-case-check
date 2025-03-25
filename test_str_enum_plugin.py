import ast
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

def test_find_invalid_str_enums():
    """Test our plugin's AST parsing logic directly"""
    from str_enum_plugin import StrEnumVisitor
    
    # Parse this file
    with open(__file__, 'r') as f:
        tree = ast.parse(f.read())
    
    visitor = StrEnumVisitor()
    visitor.visit(tree)
    
    # Debug output
    print("Found errors:", visitor.errors)
    
    # Should find 3 invalid members across 2 classes
    assert len(visitor.errors) == 3
    
    # Check error details
    assert "Invalid" in visitor.errors[0][0] and "a" in visitor.errors[0][0]
    assert "Invalid" in visitor.errors[1][0] and "b" in visitor.errors[1][0]
    assert "QualifiedInvalid" in visitor.errors[2][0]

if __name__ == "__main__":
    test_find_invalid_str_enums()