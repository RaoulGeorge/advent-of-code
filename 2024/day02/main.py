def q1(data):
    sum = 0
    for datum in data:
        if all(
            datum[i] < datum[i + 1] and abs(datum[i] - datum[i + 1]) < 4
            for i in range(len(datum) - 1)
        ):
            sum = sum + 1
        elif all(
            datum[i] > datum[i + 1] and abs(datum[i] - datum[i + 1]) < 4
            for i in range(len(datum) - 1)
        ):
            sum = sum + 1

    print(sum)


def q2(data):
    def isValid(datum, start):
        for i in range(len(datum) - 1):
            if not 1 <= datum[i] - datum[i + 1] <= 3:
                return start and any(
                    isValid(datum[j - 1 : j] + datum[j + 1 :], False)
                    for j in (i, i + 1)
                )
        return True

    result = sum(isValid(datum, True) or isValid(datum[::-1], True) for datum in data)
    print(result)


data = [[*map(int, line.split())] for line in open("./2024/day02/input.txt")]

q1(data)
q2(data)
