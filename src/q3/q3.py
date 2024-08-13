import re
from dataclasses import dataclass

FILE = "input.txt"


@dataclass
class Number:
    row: int
    start: int  # inclusive
    end: int  # exlusice
    value: int


def main():
    with open(FILE, "r") as f:
        map = f.read().splitlines()

    numbers: list[Number] = []
    # hashes of coordinates
    symbols: set[str] = set()
    # preprocessing
    for row, line in enumerate(map):
        # numbers
        for number in re.finditer(r"[0-9]+", line):
            start, end = number.span()
            value = int(number.group(0))
            numbers.append(Number(row, start, end, value))

        # symbols
        for j, char in enumerate(line):
            if not char.isdigit() and char != '.':
                symbols.add(myhash(row, j))

    # compute answer
    result = 0
    for number in numbers:
        if is_adjacent(symbols, number):
            result += number.value

    print(f"answer : {result}")


def myhash(x: int, y: int) -> str:
    return f"{x},{y}"


def is_adjacent(symbols: set[str], number: Number) -> bool:
    for j in range(number.start - 1, number.end + 1):
        above = myhash(number.row - 1, j)
        below = myhash(number.row + 1, j)
        if above in symbols or below in symbols:
            return True

    left = myhash(number.row, number.start - 1)
    right = myhash(number.row, number.end)
    return left in symbols or right in symbols


if __name__ == "__main__":
    main()
