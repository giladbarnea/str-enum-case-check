import ast

class StrEnumVisitor(ast.NodeVisitor):
    def __init__(self):
        self.errors = []
        
    def visit_ClassDef(self, node):
        bases = []
        
        # Collect base class names
        for base in node.bases:
            if isinstance(base, ast.Name) and base.id == "StrEnum":
                bases.append("StrEnum")
            elif isinstance(base, ast.Attribute) and isinstance(base.value, ast.Name):
                if base.value.id == "enum" and base.attr == "StrEnum":
                    bases.append("enum.StrEnum")
        
        # If this class inherits from StrEnum
        if "StrEnum" in bases or "enum.StrEnum" in bases:
            # Check each assignment in the class body
            for item in node.body:
                if isinstance(item, ast.Assign) and len(item.targets) == 1:
                    target = item.targets[0]
                    if isinstance(target, ast.Name):
                        # Skip auto() calls
                        if isinstance(item.value, ast.Call) and (
                            (isinstance(item.value.func, ast.Name) and item.value.func.id == "auto") or
                            (isinstance(item.value.func, ast.Attribute) and 
                             isinstance(item.value.func.value, ast.Name) and
                             item.value.func.value.id == "enum" and 
                             item.value.func.attr == "auto")
                        ):
                            continue
                            
                        # Check for string literals (only direct string constants, not expressions)
                        if isinstance(item.value, ast.Constant) and isinstance(item.value.value, str):
                            name = target.id
                            value = item.value.value
                            
                            # Compare name and value casing
                            if name != value and name.lower() == value.lower():
                                self.errors.append((
                                    f"SE001 StrEnum '{node.name}' has member '{name}' with inconsistent casing: value is '{value}'",
                                    item.lineno,
                                    item.col_offset
                                ))
                        # Ignore complex expressions - we only check direct string assignments
        
        self.generic_visit(node)

class Plugin:
    name = "flake8-str-enum"
    version = "0.1.0"
    
    def __init__(self, tree, filename=""):
        self.tree = tree
        self.filename = filename
        
    def run(self):
        visitor = StrEnumVisitor()
        visitor.visit(self.tree)
        
        for msg, line, col in visitor.errors:
            yield line, col, msg, type(self)