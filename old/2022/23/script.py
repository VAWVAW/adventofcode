
TEST = False


directions = [((0, -1), (1, -1), (-1, -1)), ((0, 1), (-1, 1), (1, 1)), ((-1, 0), (-1, -1), (-1, 1)), ((1, 0), (1, -1), (1, 1))]
adj = ((-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1))


def get(world, x, y):
    if y not in world:
        world[y] = {}
    if x not in world[y]:
        world[y][x] = "."
        return "."
    return world[y][x]


class Elve:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
        self.prop = None
        self.do_move = False

    def propose(self, world):
        for a in adj:
            if get(world, self.x + a[0], self.y + a[1]) == "#":
                break
        else:
            self.do_move = False
            return

        for direction in directions:
            if get(world, self.x + direction[1][0], self.y + direction[1][1]) == "#":
                continue
            if get(world, self.x + direction[2][0], self.y + direction[2][1]) == "#":
                continue
            n = get(world, self.x + direction[0][0], self.y + direction[0][1])
            if n == "#":
                continue

            self.prop = (self.x + direction[0][0], self.y + direction[0][1])
            if n == ".":
                world[self.y + direction[0][1]][self.x + direction[0][0]] = "P"
            elif n == "P":
                world[self.y + direction[0][1]][self.x + direction[0][0]] = "B"
            self.do_move = True
            break
        else:
            self.do_move = False

    def move(self, world):
        if not self.do_move:
            return
        if world[self.prop[1]][self.prop[0]] == "P":
            world[self.y][self.x] = "."
            self.x, self.y = self.prop
            world[self.prop[1]][self.prop[0]] = "#"


def print_world(world):
    min_x = max_x = 0
    for _, line in world.items():
        min_x = min(min_x, min(line))
        max_x = max(max_x, max(line))
    for y in range(min(world), max(world)+1):
        line = world[y]
        for i in range(min_x, max_x+1):
            print(line[i] if i in line else ".", end="")
        print()
    print()


def clear_world(world):
    for y in world:
        for x in world[y]:
            if world[y][x] == "B":
                world[y][x] = "."


def calc_area(world, n_elves: int):
    min_y = max_y = min_x = max_x = 0
    for y, line in world.items():
        for x, char in line.items():
            if char == "#":
                min_x = min(min_x, x)
                max_x = max(max_x, x)
                min_y = min(min_y, y)
                max_y = max(max_y, y)

    print((max_x - min_x+1)*(max_y - min_y+1)-n_elves)


def main():
    world = {}
    elves = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for y, line in enumerate(ifile.readlines()):
            line = line.strip()
            world[y] = {}
            for x, char in enumerate(line):
                world[y][x] = char
                if char == "#":
                    elves.append(Elve(x, y))

    i = 0
    while True:
        did_move = False
        for elve in elves:
            elve.propose(world)
        for elve in elves:
            elve.move(world)
            if elve.do_move:
                did_move = True
        directions.append(directions.pop(0))
        clear_world(world)
        i += 1
        if i == 10:
            calc_area(world, len(elves))
        if not did_move:
            print(i)
            break


if __name__ == "__main__":
    main()
