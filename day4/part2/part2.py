import sys


class Board:
    def __init__(self, input):
        board = self._create_board(input)
        self.rows = list(map(set, board))
        self.columns = list(map(set, zip(*board)))
        self.has_won = False

    def _create_board(self, input):
        return list(map(list, (map(int, val) for val in map(str.split, input.splitlines()))))
    
    def won(self, drawn):
        return (any(column <= drawn for column in self.columns) or
                any(row <= drawn for row in self.rows))
    
    def sum_all_unmarked(self, drawn):
        return sum(sum(row - drawn) for row in self.rows)


def load(path):
    with open(path, 'r') as file:
        contents = file.read().split("\n\n")
        numbers = list(map(int, contents[0].split(",")))
        boards = list(map(lambda input: Board(input), contents[1:]))
        return numbers, boards


def play_bingo(input):
    drawn = set()
    scores = []
    numbers, boards = input
    for number in numbers:
        drawn.add(number)
        for board in boards:
            if not board.has_won and board.won(drawn):
                scores.append(board.sum_all_unmarked(drawn) * number)
                board.has_won =  True
        if len(scores) == len(boards):
            return scores[-1]


def main():
    path = sys.argv[1]
    result = play_bingo(load(path))
    print(result)


if __name__ == "__main__":
    main()
