def is_valid(string):
    return string == "XMAS"


def is_really_valid(string):
    return string == "MAS" or string == "SAM"


def q1():
    with open("./2024/day04/input.txt") as content:
        lines = content.read().split("\n")
        count = 0

        for i in range(len(lines)):
            for j in range(len(lines[i])):
                # horizontal (L -> R)
                if j + 4 <= len(lines[i]):
                    sequence = "".join(lines[i][j : j + 4])
                    if is_valid(sequence):
                        count += 1

                # horizontal (R -> L)
                if j - 3 >= 0:
                    sequence = "".join(lines[i][j : j - 4 : -1])
                    if is_valid(sequence):
                        count += 1

                # vertical (T -> B)
                if i + 4 <= len(lines):
                    sequence = "".join(lines[x][j] for x in range(i, i + 4))
                    if is_valid(sequence):
                        count += 1

                # vertical (B -> T)
                if i - 3 >= 0:
                    sequence = "".join(lines[x][j] for x in range(i, i - 4, -1))
                    if is_valid(sequence):
                        count += 1

                # diagonal (TL -> BR)

                if i + 4 <= len(lines) and j + 4 <= len(lines[i]):
                    sequence = "".join(lines[i + k][j + k] for k in range(4))
                    if is_valid(sequence):
                        count += 1

                # diagonal (BR -> TL)
                if i - 3 >= 0 and j - 3 >= 0:
                    sequence = "".join(lines[i - k][j - k] for k in range(4))
                    if is_valid(sequence):
                        count += 1

                # diagonal (TR -> BL)
                if i + 4 <= len(lines) and j - 3 >= 0:
                    sequence = "".join(lines[i + k][j - k] for k in range(4))
                    if is_valid(sequence):
                        count += 1

                # diagonal (BL -> TR)
                if i - 3 >= 0 and j + 4 <= len(lines[i]):
                    sequence = "".join(lines[i - k][j + k] for k in range(4))
                    if is_valid(sequence):
                        count += 1

        print(count)


def q2():
    with open("./2024/day04/input.txt") as content:
        lines = content.read().split("\n")
        count = 0

        for i in range(len(lines)):
            for j in range(len(lines[i])):
                if lines[i][j] == "A":
                    if (
                        0 <= i - 1
                        and i + 1 < len(lines)
                        and 0 <= j - 1
                        and j + 1 < len(lines[i])
                    ):
                        diagonals = [
                            "".join(
                                [lines[i - 1][j - 1], lines[i][j], lines[i + 1][j + 1]]
                            ),
                            "".join(
                                [lines[i - 1][j + 1], lines[i][j], lines[i + 1][j - 1]]
                            ),
                        ]
                        if all(is_really_valid(diagonal) for diagonal in diagonals):
                            count += 1
        print(count)


q1()
q2()
