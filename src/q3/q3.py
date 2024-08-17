import re
from dataclasses import dataclass

import pytest

FILE = "input.txt"


def main():
    part_2()


@dataclass
class Number:
    row: int
    start: int  # inclusive
    end: int  # exlusice
    value: int


def part_1():
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


def part_2():
    """map:
    "x,y" -> number,
    so
    123 -> {"0,0" : 0, "0,1" : 0, "0,2" : 0}
    too annoying to identify unique numbers, instead we can do this
    [123]
     0
    so we store indices of numbers, then maintain a unique set of these
    for all '*', check if there is an adjacdent number"""

    with open(FILE, "r") as f:
        result = solve_part_2(f.read().splitlines())
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


def solve_part_2(map: list[str]) -> int:
    locations, numbers = preprocess(map)

    result = 0
    # asterisks
    for row, line in enumerate(map):
        for col, chr in enumerate(line):
            if chr == "*":
                adj = adjacdent_numbers(row, col, locations)
                if len(adj) == 2:
                    a, b = list(adj)
                    result += numbers[a] * numbers[b]

    return result


def preprocess(map: list[str]) -> tuple[dict[str, int], list[int]]:
    """Return locations, numbers"""
    locations: dict[str, int] = dict()
    numbers: list[int] = []

    # preprocessing
    for row, line in enumerate(map):
        # numbers
        for number in re.finditer(r"[0-9]+", line):
            start, end = number.span()
            value = int(number.group(0))
            for col in range(start, end):
                locations[myhash(col, row)] = len(numbers)
            numbers.append(value)

    return locations, numbers


def adjacdent_numbers(row: int, col: int, locations: dict[str, int]) -> set[int]:
    """Find indices of all adjacdent numbers for a given position"""
    result: set[int] = set()
    offsets: list[tuple[int, int]] = [
        # above
        (-1, -1), (0, -1), (1, -1),
        # left, right
        (-1, 0), (1, 0),
        # below
        (-1, 1), (0, 1), (1, 1)
    ]
    for offset in offsets:
        dx, dy = offset
        pos = myhash(col + dx, row + dy)
        if pos in locations:
            result.add(locations[pos])

    return result


def test_adjacdent_numbers():
    indices = adjacdent_numbers(1, 1, {
        "0,0": 0, "1,0": 1, "2,0": 2,
        "0,1": 3, "2,1": 4,
        "0,2": 5, "1,2": 6, "2,2": 7
    })
    assert indices == set([i for i in range(8)])


def test_part_2_small():
    map = ["467..114..",
           "...*......",
           "..35..633.",
           "......#...",
           "617*......",
           ".....+.58.",
           "..592.....",
           "......755.",
           "...$.*....",
           ".664.598.."]
    locations, _ = preprocess(map)
    assert (map[1][3] == "*")
    assert adjacdent_numbers(1, 3, locations) == set([0, 2])
    assert solve_part_2(map) == 467835


if __name__ == "__main__":
    main()
