import sys


def load(path):
    with open(path, 'r') as file:
        return file.readline().strip()


def parse_literal(packet):
    moved_by = 0
    while True:
        if packet[moved_by] == '0': break
        moved_by += 5
    
    return moved_by + 5


def parse(packet):
    versions_sum = int(packet[0:3], 2)
    type_id = int(packet[3:6], 2)

    moved_by = 6

    if type_id == 4:
        moved_by += parse_literal(packet[moved_by:])
        return versions_sum, moved_by
    else:
        length_type = packet[moved_by]
        moved_by += 1

        if length_type == '0':
            length = int(packet[moved_by:moved_by + 15], 2)
            moved_by += 15

            while length > 0:
                versions, moves = parse(packet[moved_by:])
                moved_by += moves
                length -= moves
                versions_sum += versions
        else:
            number_of_packets = int(packet[moved_by:moved_by + 11], 2)
            moved_by += 11

            while number_of_packets > 0:
                versions, moves = parse(packet[moved_by:])
                moved_by += moves
                versions_sum += versions
                number_of_packets -= 1
    
    return versions_sum, moved_by


def main():
    path = sys.argv[1]
    hex_packet = load(path)
    binary_packet = bin(int(hex_packet, 16))[2:].zfill(len(hex_packet * 4))
    result, _ = parse(binary_packet)
    print(result)


if __name__ == "__main__":
    main()
