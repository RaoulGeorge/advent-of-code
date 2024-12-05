from functools import cmp_to_key


def cmp(a, b, order_rules):
    for x, y in order_rules:
        if a == x and b == y:
            return -1
        elif a == y and b == x:
            return 1
    return 0


def validate_or_fix(order_rules, updates):
    valid_updates = []
    fixed_updates = []

    for update in updates:
        sorted_update = sorted(
            update, key=cmp_to_key(lambda a, b: cmp(a, b, order_rules))
        )
        if sorted_update == update:
            valid_updates.append(update)
        else:
            fixed_updates.append(sorted_update)

    return valid_updates, fixed_updates


def get_sum(updates):
    sum = 0
    for update in updates:
        sum += update[(len(update) - 1) // 2]

    print(sum)


with open("./2024/day05/input1.txt") as file:
    order_rules = [tuple(map(int, line.strip().split("|"))) for line in file]

with open("./2024/day05/input2.txt") as file:
    updates = [list(map(int, line.strip().split(","))) for line in file]

valid_updates, fixed_updates = validate_or_fix(order_rules, updates)

# q1
get_sum(valid_updates)

# q2
get_sum(fixed_updates)
