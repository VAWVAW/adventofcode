
score = 0
with open("four.txt") as ifile:
    for line in ifile.readlines():
        areas = [[int(y) for y in x.split("-")] for x in line.strip().split(",")]

        if (areas[0][0] <= areas[1][0]) == (areas[0][1] >= areas[1][1]):
            score += 1
            continue
        if (areas[0][0] >= areas[1][0]) == (areas[0][1] <= areas[1][1]):
            score += 1
print(score)

# two

score = 0
with open("four.txt") as ifile:
    for line in ifile.readlines():
        areas = [[int(y) for y in x.split("-")] for x in line.strip().split(",")]

        if areas[0][1] >= areas[1][0] and areas[1][1] >= areas[0][0]:
            score += 1
            continue
        if areas[1][1] >= areas[0][0] and areas[0][1] >= areas[1][0]:
            score += 1
print(score)
