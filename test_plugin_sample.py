import ast
from str_enum_plugin import Plugin

def test_plugin_with_sample():
    with open('/private/tmp/str_enum_flake8_plugin/sample.py', 'r') as f:
        tree = ast.parse(f.read())
    
    plugin = Plugin(tree, '/private/tmp/str_enum_flake8_plugin/sample.py')
    errors = list(plugin.run())
    
    # Should find 2 errors (one in each class)
    assert len(errors) == 2
    
    # Print for debugging
    for line, col, msg, _ in errors:
        print(f"Line {line}, Col {col}: {msg}")
    
    # Check specific error messages
    assert any(line == 6 and "Colors" in msg and "red" in msg for line, col, msg, _ in errors)
    assert any(line == 12 and "Status" in msg and "success" in msg for line, col, msg, _ in errors)

if __name__ == "__main__":
    test_plugin_with_sample()