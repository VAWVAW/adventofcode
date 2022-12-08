import re

field = []

with open("data.txt") as ifile:
    for line in ifile.readlines():
        field.append([])
        for digit in re.findall(r'\d', line):
            field[-1].append((digit, False, 0))


for i in range(len(field)):
    for j in range(len(field[i])):
        is_visible = False
        score = 1
        this = field[i][j][0]

        x = i - 1
        score_d = 0
        while x >= 0:
            score_d += 1
            if field[x][j][0] >= this:
                break
            x -= 1
        else:
            is_visible = True
        score *= score_d

        x = i + 1
        score_d = 0
        while x <= len(field) - 1:
            score_d += 1
            if field[x][j][0] >= this:
                break
            x += 1
        else:
            is_visible = True
        score *= score_d

        x = j - 1
        score_d = 0
        while x >= 0:
            score_d += 1
            if field[i][x][0] >= this:
                break
            x -= 1
        else:
            is_visible = True
        score *= score_d

        x = j + 1
        score_d = 0
        while x <= len(field[j]) - 1:
            score_d += 1
            if field[i][x][0] >= this:
                break
            x += 1
        else:
            is_visible = True
        score *= score_d

        field[i][j] = (this, is_visible, score)

c = 0
max_score = 0
for i in range(len(field)):
    for j in range(len(field[i])):
        if field[i][j][1]:
            c += 1
        if field[i][j][2] > max_score:
            max_score = field[i][j][2]

print(c)
print(max_score)
