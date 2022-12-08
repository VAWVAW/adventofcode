import re

field = []

with open("eight_test.txt") as ifile:
    for line in ifile.readlines():
        field.append([])
        for digit in re.findall(r'\d', line):
            field[-1].append((digit, False))


for i in range(len(field)):
    for j in range(len(field[i])):
        is_visible = False
        this = field[i][j][0]

        x = i - 1
        while x >= 0:
            if field[x][j][0] >= this:
                break
            x -= 1
        else:
            is_visible = True

        x = i + 1
        while x <= len(field) - 1:
            if field[x][j][0] >= this:
                break
            x += 1
        else:
            is_visible = True

        x = j - 1
        while x >= 0:
            if field[i][x][0] >= this:
                break
            x -= 1
        else:
            is_visible = True

        x = j + 1
        while x <= len(field[j]) - 1:
            if field[j][x][0] >= this:
                break
            x += 1
        else:
            is_visible = True

        field[i][j] = (this, is_visible)

c = 0
for i in range(len(field)):
    for j in range(len(field[i])):
        if field[i][j][1]:
            c += 1
        print("1" if field[i][j][1] else "0", end="")
    print()
print(c)
