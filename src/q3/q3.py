import re

FILE = "input.txt"

with open(FILE, "r") as f:
    map = f.read().splitlines()

total = 0
for row, line in enumerate(map):
    for number in re.finditer(r"[0-9]+", line):
        start, end = number.span()


def is_adjacent(board: list[str], row, start: int, end: int) -> bool:
    if row > 0:
        above = board[row - 1][max(0, start - 1):min(len(line), end)]
        if has_symbols(above):
            return True
    if row < len(board) - 1:
        below = board[row + 1][max(0, start - 1):min(len(line), end)]
        if has_symbols(below):
            return True
    # left col
    # right col


def has_symbols(string: str) -> bool:
    return any(not (s.isdigit() and s == '.') for s in string)
