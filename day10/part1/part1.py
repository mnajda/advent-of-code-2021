import sys


OPENING = ['(', '[', '{', '<']
MAPPING = {')': '(', ']': '[', '}': '{', '>': '<'}
SCORES = {')': 3, ']': 57, '}': 1197, '>': 25137}


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
                return SCORES[char]
    
    return 0


def solve(input):
    return sum([validate(expr) for expr in input])


def main():
    path = sys.argv[1]
    result = solve(load(path))
    print(result);


if __name__ == "__main__":
    main()
