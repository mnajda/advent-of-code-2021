import sys
from functools import reduce


OPENING = ['(', '[', '{', '<']
MAPPING = {')': '(', ']': '[', '}': '{', '>': '<'}
REVERSED_MAPPING = {'(': ')', '[': ']', '{': '}', '<': '>'}
SCORES = {')': 1, ']': 2, '}': 3, '>': 4}


def load(path):
    with open(path, 'r') as file:
        return file.read().split("\n")


def validate(expr):
    stack = []
    for char in expr:
        if char in OPENING:
            stack.append(char)
        else:
            opening = stack.pop()
            expected = MAPPING[char]
            if opening != expected:
                return 0
    
    completion = [REVERSED_MAPPING[char] for char in stack[::-1]]
    return reduce(lambda acc, char: (acc * 5) + SCORES[char], completion, 0)


def solve(input):
    scores = list(filter(lambda score: score != 0, [validate(expr) for expr in input]))
    return sorted(scores)[len(scores) // 2]


def main():
    path = sys.argv[1]
    result = solve(load(path))
    print(result);


if __name__ == "__main__":
    main()
