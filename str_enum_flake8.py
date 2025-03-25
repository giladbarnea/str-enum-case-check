"""
flake8 plugin to validate that StrEnum member names match their string values.
"""

import ast
import enum
from typing import Any, Generator, List, Tuple, Union

# Rule code for this plugin
SE001 = 'SE001'  # StrEnum member name case doesn't match value


class StrEnumVisitor(ast.NodeVisitor):
    """AST visitor that finds StrEnum classes and validates their member names."""
    
    def __init__(self):
        self.errors: List[Tuple[int, int, str]] = []
    
    def visit_ClassDef(self, node: ast.ClassDef) -> None:
        """Visit a class definition and check if it's a StrEnum subclass."""
        is_str_enum = False
        
        for base in node.bases:
            if isinstance(base, ast.Name) and base.id == "StrEnum":
                is_str_enum = True
                break
            elif isinstance(base, ast.Attribute) and isinstance(base.value, ast.Name):
                if base.value.id == "enum" and base.attr == "StrEnum":
                    is_str_enum = True
                    break
                    
        if is_str_enum:
            self._check_str_enum_members(node)
            
        # Continue visiting other nodes
        self.generic_visit(node)
    
    def _check_str_enum_members(self, node: ast.ClassDef) -> None:
        """Check that StrEnum member names match their string values."""
        for item in node.body:
            if isinstance(item, ast.Assign) and len(item.targets) == 1:
                target = item.targets[0]
                if isinstance(target, ast.Name):
                    # Skip auto() calls
                    if self._is_auto_call(item.value):
                        continue
                        
                    # Check for string literals
                    if isinstance(item.value, ast.Constant) and isinstance(item.value.value, str):
                        name = target.id
                        value = item.value.value
                        
                        # Compare name and value casing
                        if name != value and name.lower() == value.lower():
                            self.errors.append((
                                item.lineno,
                                item.col_offset,
                                f"{SE001} StrEnum '{node.name}' has member '{name}' with inconsistent casing: value is '{value}'"
                            ))

    def _is_auto_call(self, node: ast.expr) -> bool:
        """Check if the node is an auto() or enum.auto() call."""
        if not isinstance(node, ast.Call):
            return False
            
        if isinstance(node.func, ast.Name) and node.func.id == "auto":
            return True
            
        if (isinstance(node.func, ast.Attribute) and 
                isinstance(node.func.value, ast.Name) and
                node.func.value.id == "enum" and 
                node.func.attr == "auto"):
            return True
            
        return False


class Plugin:
    """flake8 plugin to check StrEnum member names."""
    
    name = "str_enum_casing"
    version = "0.1.0"
    
    def __init__(self, tree: ast.AST):
        self.tree = tree
        
    def run(self) -> Generator[Tuple[int, int, str, type], None, None]:
        """Run the check on the tree."""
        visitor = StrEnumVisitor()
        visitor.visit(self.tree)
        
        for line, col, message in visitor.errors:
            yield line, col, message, type(self)