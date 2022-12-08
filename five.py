import re

data = [None]
with open("five_data.txt") as ifile:
    for line in ifile.readlines():
        l = list(line.strip())
        data.append(l[1:])

with open("five_moves.txt") as ifile:
    for line in ifile.readlines():
        action = [int(x) for x in re.findall(r'\d+', line)]
        for _ in range(action[0]):
            data[action[2]].append(data[action[1]].pop())

for pile in data:
    if pile is None:
        continue
    print(pile[-1], end="")
print()

# two
data = [None]
with open("five_data.txt") as ifile:
    for line in ifile.readlines():
        l = list(line.strip())
        data.append(l[1:])

with open("five_moves.txt") as ifile:
    for line in ifile.readlines():
        action = [int(x) for x in re.findall(r'\d+', line)]
        pile = []
        for _ in range(action[0]):
            pile.append(data[action[1]].pop())
        for _ in range(action[0]):
            data[action[2]].append(pile.pop())

for pile in data:
    if pile is None:
        continue
    print(pile[-1], end="")
print()
