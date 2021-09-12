#!/usr/bin/env python3
import sys
from pathlib import Path
from time import time


def main():
    code = Path('/home/samuel/code/pydantic/pydantic/main.py').read_text()
    load_start = time()

    import pygments
    from pygments.formatters import Terminal256Formatter
    from pygments.lexers import PythonLexer
    lexer = PythonLexer()
    formatter = Terminal256Formatter(style='vim')
    load_time = time() - load_start

    highlight_start = time()
    ansi_code = pygments.highlight(code, lexer=lexer, formatter=formatter)
    highlight_time = time() - highlight_start

    print(f'load time:          {load_time * 1000:0.2f}ms')
    print(f'highlight time:     {highlight_time * 1000:0.2f}ms')
    print(f'output ansi length: {len(ansi_code)}')
    print(f'versions pygments={pygments.__version__} python={sys.version}')


if __name__ == '__main__':
    main()
