TEST = False


def is_lava(world, x, y, z):
    if x not in world:
        return False
    if y not in world[x]:
        return False
    if z not in world[x][y]:
        return False
    return True


def missing_elements(l: list[int]) -> list[int]:
    return [i for i in range(min(l)+1, max(l)) if i not in l]


def main():
    # x y z
    world = {}
    # y z x
    world_y = {}
    # z x y
    world_z = {}
    drops = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for line in ifile.readlines():
            drop = [int(x) for x in line.strip().split(",")]
            drops.append(drop)
            x, y, z = drop

            if x not in world:
                world[x] = {}
            if y not in world[x]:
                world[x][y] = {}
            world[x][y][z] = -1

            if y not in world_y:
                world_y[y] = {}
            if z not in world_y[y]:
                world_y[y][z] = {}
            world_y[y][z][x] = -1

            if z not in world_z:
                world_z[z] = {}
            if x not in world_z[z]:
                world_z[z][x] = {}
            world_z[z][x][y] = -1

    surface = len(drops) * 6
    for drop in drops:
        x, y, z = drop
        if is_lava(world, x-1, y, z):
            surface -= 1
        if is_lava(world, x+1, y, z):
            surface -= 1
        if is_lava(world, x, y-1, z):
            surface -= 1
        if is_lava(world, x, y+1, z):
            surface -= 1
        if is_lava(world, x, y, z-1):
            surface -= 1
        if is_lava(world, x, y, z+1):
            surface -= 1

    # one
    print(surface)

    # two
    z_min = y_min = x_min = 999
    z_max = y_max = x_max = 0
    possibly_missing = {}
    for x in world:
        x_max = max(x_max, x)
        x_min = min(x_min, x)
        for y in world[x]:
            missing = missing_elements(world[x][y].keys())
            for el in missing:
                possibly_missing[(x, y, el)] = 1

    for y in world_y:
        y_max = max(y_max, y)
        y_min = min(y_min, y)
        for z in world_y[y]:
            missing = missing_elements(world_y[y][z].keys())
            for el in missing:
                if (el, y, z) in possibly_missing:
                    possibly_missing[(el, y, z)] = 2

    for z in world_z:
        z_max = max(z_max, z)
        z_min = min(z_min, z)
        for x in world_z[z]:
            missing = missing_elements(world_z[z][x].keys())
            for el in missing:
                if (x, el, z) in possibly_missing:
                    possibly_missing[(x, el, z)] = 3

    possibly_missing = [k for k, v in possibly_missing.items() if v == 3]
    world2 = [[[0 for z in range(z_min, z_max + 1)] for y in range(y_min, y_max + 1)] for x in range(x_min, x_max + 1)]
    for drop in drops:
        x, y, z = drop
        x = x - x_min
        y = y - y_min
        z = z - z_min
        world2[x][y][z] = -1
    for p in possibly_missing:
        x, y, z = p
        x = x - x_min
        y = y - y_min
        z = z - z_min
        world2[x][y][z] = 1

    changed = True
    while changed:
        changed = False
        for p in possibly_missing.copy():
            x, y, z = p
            x = x - x_min
            y = y - y_min
            z = z - z_min
            other = [world2[x - 1][y][z], world2[x + 1][y][z], world2[x][y - 1][z], world2[x][y + 1][z],
                     world2[x][y][z - 1], world2[x][y][z + 1]]
            if 0 in other:
                world2[x][y][z] = 0
                possibly_missing.remove(p)
                changed = True

    surface_air = len(possibly_missing) * 6
    for drop in possibly_missing:
        x, y, z = drop
        if not is_lava(world, x-1, y, z):
            surface_air -= 1
        if not is_lava(world, x+1, y, z):
            surface_air -= 1
        if not is_lava(world, x, y-1, z):
            surface_air -= 1
        if not is_lava(world, x, y+1, z):
            surface_air -= 1
        if not is_lava(world, x, y, z-1):
            surface_air -= 1
        if not is_lava(world, x, y, z+1):
            surface_air -= 1
    print(surface - surface_air)


if __name__ == "__main__":
    main()
