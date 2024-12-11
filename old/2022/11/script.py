from sys import argv
import re
import math


PART_ONE = False
ROUNDS = 20 if PART_ONE else 10000


class Monkey:
    def __init__(self, items: list[int], operation: str, test: int, if_true: int, if_false):
        self.activity = 0
        self.items = items
        self.lcm = 0
        self._operation = operation
        self._test = test
        self._if_true = if_true
        self._if_false = if_false

    def act(self, monkeys: list):
        for _ in range(len(self.items)):
            self.activity += 1

            old = self.items.pop(0)
            new = eval(self._operation)
            new = (new // 3) if PART_ONE else new

            new = new % self.lcm

            if new % self._test == 0:
                monkeys[self._if_true].items.append(new)
            else:
                monkeys[self._if_false].items.append(new)


def main():
    with open("data.txt" if len(argv) < 2 else argv[1]) as ifile:
        i_string = ifile.read()

    monkeys = []
    denominators = []
    for monkey in i_string.split("\n\n"):
        lines = monkey.split("\n")
        assert len(lines) == 6
        assert lines[0].startswith("Monkey ")

        items = [int(item) for item in re.findall(r'\d+', lines[1])]
        operation = lines[2].split("Operation: new = ")[1]
        test = int(re.findall(r'\d+', lines[3])[0])
        if_true = int(re.findall(r'\d+', lines[4])[0])
        if_false = int(re.findall(r'\d+', lines[5])[0])

        denominators.append(test)
        monkeys.append(Monkey(items, operation, test, if_true, if_false))

    # set lcm
    lcm = math.lcm(*denominators)
    for monkey in monkeys:
        monkey.lcm = lcm

    for c_round in range(ROUNDS):
        if c_round % 1000 == 0:
            print(f"calculating round: {c_round}")
        for monkey in monkeys:
            monkey.act(monkeys)

    activities = [monkey.activity for monkey in monkeys]
    activities.sort(reverse=True)

    print(activities[0] * activities[1])


if __name__ == "__main__":
    main()
