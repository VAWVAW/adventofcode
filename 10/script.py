from sys import argv


instructions = []
# (opt, data)
# 0: noop
# 1: addx

with open("data.txt" if len(argv) < 2 else argv[1]) as ifile:
    for line in ifile.readlines():
        if line.startswith("noop"):
            instructions.append((0, None))
            continue
        if line.startswith("addx"):
            instructions.append((1, int(line.split(" ")[1])))


X = 1
cycle = 1
ins_p = 0
since = 0
signals = []
screen = [["." for _ in range(40)] for _ in range(6)]

while True:
    if ins_p >= len(instructions):
        break
    ins = instructions[ins_p]

    if (cycle - 20) % 40 == 0:
        signals.append(cycle * X)

    # draw to screen
    column = cycle % 40
    row = int(((cycle - column) % 240) / 40)
    screen[row][column-1] = '#' if abs(X - column+1) <= 1 else ' '

    match ins[0]:
        case 0:
            ins_p += 1
        case 1:
            if since == 0:
                since = 1
            else:
                since = 0
                X += ins[1]
                ins_p += 1
    cycle += 1


for row in screen:
    for pixel in row:
        print(pixel, end="")
    print()
print(sum(signals))
