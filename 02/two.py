import os

score = 0
points = {1:{2: 6, 3: 0}, 2:{1: 0, 3: 6}, 3:{1: 6, 2: 0}}
with open("two.txt") as ifile:
    for line in ifile.readlines():
        (o, m) = line.strip().split(" ")
        o = ord(o)-64
        m = ord(m)-87
        score += m
        if o == m:
            score += 3
            continue
        score += points[o][m]
print(score)

# part 2
score = 0
points = {1:{0: 3, 2: 2}, 2:{0: 1, 2: 3}, 3:{0: 2, 2: 1}}
with open("two.txt") as ifile:
    for line in ifile.readlines():
        (o, m) = line.strip().split(" ")
        o = ord(o)-64
        m = ord(m)-88
        score += 3*m
        if m == 1:
            score += o
            continue
        score += points[o][m]

print(score)
