from sys import argv
import numpy as np

PART_ONE = False

def set_point(world: dict, x: int, y: int, value):
    if x not in world.keys():
        world[x] = {}
    world[x][y] = value

def print_world(world):
    x_min = min(list(world.keys()))
    x_max = max(list(world.keys()))
    x_n = x_max - x_min + 1
    f = []
    for x in world.keys():
        for y in world[x].keys():
            while y > len(f) - 1:
                f.append([" "] * x_n)
            f[y][x-x_min] = "#" if world[x][y] == 0 else "O"
    f[0][500-x_min] = '+'
    for line in f:
        for c in line:
            print(c, end="")
        print()

def main():
    world = {}
    with open("data.txt" if len(argv) < 2 else argv[1]) as ifile:
        for line in ifile.readlines():
            points = [np.array([int(x) for x in point.split(",")]) for point in line.split(" -> ")]
            i = 0

            set_point(world, points[0][0], points[0][1], 0)
            while i < len(points) - 1:
                curr = start = points[i]
                direction = points[i+1] - points[i]
                direction = (direction / np.linalg.norm(direction)).astype('int32')
                while not np.array_equal(curr, points[i+1]):
                    curr += direction
                    set_point(world, curr[0], curr[1], 0)
                i += 1
    num_sand = 0
    max_y = [0]
    for x in world.keys():
        if (this_max := max(list(world[x].keys()))) > max_y[0]:
            max_y[0] = this_max
    floor = max_y[0] + 2

    while True:
        x = 500
        y = 0
        while True:
            if x-1 not in world.keys():
                world[x-1] = {}
            if x+1 not in world.keys():
                world[x+1] = {}
            if not PART_ONE and y == floor - 1:
                set_point(world, x, floor, 0)
                set_point(world, x-1, floor, 0)
                set_point(world, x+1, floor, 0)

            if y+1 not in world[x].keys():
                y += 1
                continue
            if y+1 not in world[x-1].keys():
                x -= 1
                y += 1
                continue
            if y+1 not in world[x+1].keys():
                x += 1
                y += 1
                continue
            if y > floor:
                break
            world[x][y] = 1
            num_sand += 1
            break
        if y > floor:
            break
        if y == 0:
            break
    print_world(world)
    print(num_sand)
    
                

if __name__ == "__main__":
    main()
