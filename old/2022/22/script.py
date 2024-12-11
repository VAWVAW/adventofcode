import re

TEST = False

directions = ((1, 0), (0, 1), (-1, 0), (0, -1))


def one():
    world = {}
    y = 1
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        last = False
        for line in ifile.readlines():
            if line == "\n":
                last = True
                continue
            if last:
                moves = [int(move) if re.fullmatch(r'\d+', move) else move for move in re.findall(r'\d+|[RL]', line)]
                break
            line = line.strip("\n")
            world[y] = {}
            for x in range(len(line)):
                if line[x] == " ":
                    continue
                world[y][x+1] = 0 if line[x] == "." else 1
            y += 1

    # [x, y, facing]
    pos = [min(world[1]), 1, 0]
    while world[1][pos[0]] == 1:
        pos[0] = pos[0]+1

    for move in moves:
        if isinstance(move, str):
            pos[2] = (pos[2] + (1 if move == "R" else -1)) % 4
            continue

        for _ in range(move):
            if pos[2] % 2 == 0:
                new_x = pos[0] + directions[pos[2]][0]
                new_y = pos[1]

                if new_x not in world[new_y]:
                    new_x = max(world[new_y]) if new_x < min(world[new_y]) else min(world[new_y])
            else:
                new_x = pos[0]
                new_y = pos[1] + directions[pos[2]][1]

                ys = [y for y in world if new_x in world[y]]
                if new_y not in ys:
                    new_y = max(ys) if new_y < min(ys) else min(ys)

            if world[new_y][new_x] == 1:
                break
            pos[0] = new_x
            pos[1] = new_y

    print(pos[1] * 1000 + pos[0] * 4 + pos[2])


def two():
    world = {}
    y = 1
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        last = False
        for line in ifile.readlines():
            if line == "\n":
                last = True
                continue
            if last:
                moves = [int(move) if re.fullmatch(r'\d+', move) else move for move in re.findall(r'\d+|[RL]', line)]
                break
            line = line.strip("\n")
            world[y] = {}
            for x in range(len(line)):
                if line[x] == " ":
                    continue
                world[y][x+1] = 0 if line[x] == "." else 1
            y += 1

    # [x, y, facing]
    pos = [min(world[1]), 1, 0]
    while world[1][pos[0]] == 1:
        pos[0] = pos[0]+1

    for move in moves:
        if isinstance(move, str):
            pos[2] = (pos[2] + (1 if move == "R" else -1)) % 4
            continue

        for _ in range(move):
            new_rot = pos[2]
            if pos[2] % 2 == 0:
                new_x = pos[0] + directions[pos[2]][0]
                new_y = pos[1]

                if new_x not in world[new_y]:
                    if new_y <= 50:
                        if new_rot == 0:
                            new_x = 100
                            new_y = 151 - new_y
                            new_rot = 2
                        else:
                            new_x = 1
                            new_y = 151 - new_y
                            new_rot = 0
                    elif new_y <= 100:
                        if new_rot == 0:
                            new_x = new_y + 50
                            new_y = 50
                            new_rot = 3
                        else:
                            new_x = new_y - 50
                            new_y = 101
                            new_rot = 1
                    elif new_y <= 150:
                        if new_rot == 0:
                            new_x = 150
                            new_y = 151 - new_y
                            new_rot = 2
                        else:
                            new_x = 51
                            new_y = 151 - new_y
                            new_rot = 0
                    else:
                        if new_rot == 0:
                            new_x = new_y - 100
                            new_y = 150
                            new_rot = 3
                        else:
                            new_x = new_y - 100
                            new_y = 1
                            new_rot = 1

            else:
                new_x = pos[0]
                new_y = pos[1] + directions[pos[2]][1]

                ys = [y for y in world if new_x in world[y]]
                if new_y not in ys:
                    if new_x <= 50:
                        if new_rot == 1:
                            new_y = 1
                            new_x = new_x + 100
                            new_rot = 1
                        else:
                            new_y = new_x + 50
                            new_x = 51
                            new_rot = 0
                    elif new_x <= 100:
                        if new_rot == 1:
                            new_y = new_x + 100
                            new_x = 50
                            new_rot = 2
                        else:
                            new_y = new_x + 100
                            new_x = 1
                            new_rot = 0
                    else:
                        if new_rot == 1:
                            new_y = new_x - 50
                            new_x = 100
                            new_rot = 2
                        else:
                            new_y = 200
                            new_x = new_x - 100
                            new_rot = 3

            if world[new_y][new_x] == 1:
                break
            pos[0] = new_x
            pos[1] = new_y
            pos[2] = new_rot

    print(pos[1] * 1000 + pos[0] * 4 + pos[2])


if __name__ == "__main__":
    two()
