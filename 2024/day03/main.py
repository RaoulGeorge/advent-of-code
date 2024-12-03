import re


def q1():
    with open("./2024/day03/input.txt", "r") as file:
        print(calc(file.read()))


def q2():
    with open("./2024/day03/input.txt", "r") as file:
        content = file.read()
    do_indices = []
    for match in re.finditer("do", content):
        start_do = match.start()
        do_indices.append(start_do)
    sum = 0
    dont_index = content.find("don't", 0, do_indices[0])
    sum = sum + calc(content[0 : dont_index if dont_index != -1 else do_indices[0]])
    for i in range(len(do_indices) - 1):
        # is there a don't in there?
        dont_index = content.find("don't", do_indices[i], do_indices[i + 1])
        sum = sum + calc(
            content[
                do_indices[i] : do_indices[i + 1] if dont_index == -1 else dont_index
            ]
        )
    print(sum)


def calc(content):
    instances = re.findall("mul\(\d{1,3},\d{1,3}\)", content)
    sum = 0
    for instance in instances:
        numbers = re.findall(r"\d{1,3}", instance)
        num1, num2 = map(int, numbers)
        sum = sum + (num1 * num2)

    return sum


q1()
q2()
