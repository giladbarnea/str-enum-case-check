import ast
from str_enum_flake8 import Plugin

def main():
    with open('test_examples.py', 'r') as f:
        tree = ast.parse(f.read())
    
    plugin = Plugin(tree)
    for error in plugin.run():
        print(f'{error[0]}:{error[1]}: {error[2]}')

if __name__ == "__main__":
    main()