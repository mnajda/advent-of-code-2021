import sys
from math import prod


def load(path):
    with open(path, 'r') as file:
        return file.readline().strip()


def parse_literal(packet):
    moved_by = 0
    value = ""
    while True:
        value += packet[moved_by + 1:moved_by + 5]
        if packet[moved_by] == '0': break
        moved_by += 5
    
    return moved_by + 5, int(value, 2)


OPERATIONS = {
    0: sum,
    1: prod,
    2: min,
    3: max,
    4: None,
    5: lambda values: int(values[0] > values[1]),
    6: lambda values: int(values[0] < values[1]),
    7: lambda values: int(values[0] == values[1])}


def parse(packet):
    versions_sum = int(packet[0:3], 2)
    type_id = int(packet[3:6], 2)

    moved_by = 6

    if type_id == 4:
        moves, value = parse_literal(packet[moved_by:])
        moved_by += moves
        return versions_sum, moved_by, value

    all_values = []
    length_type = packet[moved_by]
    moved_by += 1

    if length_type == '0':
        length = int(packet[moved_by:moved_by + 15], 2)
        moved_by += 15
        while length > 0:
            versions, moves, values = parse(packet[moved_by:])
            moved_by += moves
            length -= moves
            versions_sum += versions
            all_values.append(values)
    else:
        number_of_packets = int(packet[moved_by:moved_by + 11], 2)
        moved_by += 11
        while number_of_packets > 0:
            versions, moves, values = parse(packet[moved_by:])
            moved_by += moves
            versions_sum += versions
            number_of_packets -= 1
            all_values.append(values)
    
    return versions_sum, moved_by, OPERATIONS[type_id](all_values)


def main():
    path = sys.argv[1]
    hex_packet = load(path)
    binary_packet = bin(int(hex_packet, 16))[2:].zfill(len(hex_packet * 4))
    _, _, result = parse(binary_packet)
    print(result)


if __name__ == "__main__":
    main()
