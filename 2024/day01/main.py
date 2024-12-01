def q1():
    with open("./2024/day01/input.txt") as lines:
        left = []
        right = []
        for line in lines:
            x, y = line.strip().split("   ")
            left.append(int(x))
            right.append(int(y))

    left.sort()
    right.sort()
    sum = 0
    for l, r in zip(left, right):
        sum += abs(l - r)

    print(sum)


def q2():
    with open("./2024/day01/input.txt") as lines:
        left = []
        right_hashmap = {}
        for line in lines:
            x, y = line.strip().split("   ")
            left.append(x)
            if y not in right_hashmap:
                right_hashmap[y] = 1
            else:
                right_hashmap[y] = right_hashmap[y] + 1
    similarity_score = 0
    for l in left:
        if l in right_hashmap:
            similarity_score = similarity_score + (int(l) * int(right_hashmap[l]))
    print(similarity_score)


q1()
q2()
