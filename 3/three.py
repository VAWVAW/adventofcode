
score = 0
shared = []
with open("three.txt") as ifile:
    for line in ifile.readlines():
        line = line.strip()
        half = int(len(line)/2)
        c1 = line[:half]
        c2 = line[half:]
        used = []
        for c in c1:
            if c in c2 and c not in used:
                shared.append(c)
                used.append(c)

for c in shared:
    op = ord(c) - 38
    if op > 52:
        op -= 58
    score += op

print(score)

# two
score = 0
items = []
with open("three.txt") as ifile:
    while True:
        l1 = ifile.readline().strip()
        if l1 == "":
            break
        candidates = []
        l1 = list(l1)
        l2 = list(ifile.readline())
        l3 = list(ifile.readline())

        for c in l1:
            if c in l2 and c in l3:
                items.append(c)
                break

for c in items:
    op = ord(c) - 38
    if op > 52:
        op -= 58
    score += op

print(score)
