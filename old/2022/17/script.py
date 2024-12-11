TEST = False
PART_ONE = False
MAX_ROCKS = 1000000000000


# line represented binary (the lowest bit is left); mirrored top down
class Rock:
    def __init__(self, rock_type: int):
        self.rock_type = rock_type
        match rock_type:
            case 0:
                self.width = 4
                self.height = 1
                self.shape = [15]
            case 1:
                self.width = 3
                self.height = 3
                self.shape = [2, 7, 2]
            case 2:
                self.width = 3
                self.height = 3
                self.shape = [7, 4, 4]
            case 3:
                self.width = 1
                self.height = 4
                self.shape = [1, 1, 1, 1]
            case 4:
                self.width = 2
                self.height = 2
                self.shape = [3, 3]


def one():
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        airflow = ifile.read()
    len_airflow = len(airflow)
    rocks = [Rock(i) for i in range(5)]
    len_rocks = len(rocks)
    world = {0: 127}

    airflow_counter = 0
    max_height = 0
    for i in range(2022):

        def check_collision():
            for line in range(rock.height):
                if height + line > max_height:
                    return False
                if (rock.shape[line] << x) & world[height + line] != 0:
                    # has collided
                    return True
            return False

        x = 2
        height = max_height + 4
        rock = rocks[i % len_rocks]
        # falls are free
        for _ in range(3):
            old_x = x
            x += 1 if airflow[airflow_counter % len_airflow] == '>' else -1
            airflow_counter += 1
            if x < 0 or x > 7 - rock.width:
                x = old_x
            height -= 1
        # fall and check
        while True:
            old_x = x
            x += 1 if airflow[airflow_counter % len_airflow] == '>' else -1
            airflow_counter += 1
            if x < 0 or x > 7 - rock.width:
                x = old_x
            elif check_collision():
                x = old_x
            old_height = height
            height -= 1
            if check_collision():
                height = old_height
                # place rock
                if height + rock.height - 1 > max_height:
                    for line in range(max_height + 1, height + rock.height):
                        world[line] = 0
                    max_height = height + rock.height - 1
                for line in range(rock.height):
                    world[height + line] |= rock.shape[line] << x
                break

    print(max(world))


def two():
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        airflow = ifile.read()
    len_airflow = len(airflow)
    rocks = [Rock(i) for i in range(5)]
    len_rocks = len(rocks)
    world = {0: 127}
    rock_num = 0

    airflow_counter = 0
    max_height = 0
    i = 0

    def fall_rock():
        nonlocal airflow_counter
        nonlocal max_height
        nonlocal i

        def check_collision():
            for line in range(rock.height):
                if height + line > max_height:
                    return False
                if (rock.shape[line] << x) & world[height + line] != 0:
                    # has collided
                    return True
            return False

        x = 2
        height = max_height + 4
        rock = rocks[i % len_rocks]
        # falls are free
        for _ in range(3):
            old_x = x
            x += 1 if airflow[airflow_counter % len_airflow] == '>' else -1
            airflow_counter += 1
            if x < 0 or x > 7 - rock.width:
                x = old_x
            height -= 1
        # fall and check
        while True:
            old_x = x
            x += 1 if airflow[airflow_counter % len_airflow] == '>' else -1
            airflow_counter += 1
            if x < 0 or x > 7 - rock.width:
                x = old_x
            elif check_collision():
                x = old_x
            old_height = height
            height -= 1
            if check_collision():
                height = old_height
                # place rock
                if height + rock.height - 1 > max_height:
                    for line in range(max_height + 1, height + rock.height):
                        world[line] = 0
                    max_height = height + rock.height - 1
                for line in range(rock.height):
                    world[height + line] |= rock.shape[line] << x
                break
        i += 1

    for _ in range(100000):
        fall_rock()
    rock_num += 100000

    repeat_height = None
    for d in range(1, 10000):
        for j in range(50000, 100000):
            if (world[j] ^ world[j + d]) != 0:
                break
        else:
            repeat_height = d
            break
    repeat_start = 0
    while True:
        if (world[repeat_start] ^ world[repeat_start + repeat_height]) == 0:
            break
        repeat_start += 1
    repeat_shape = {x: world[max_height - x] for x in range(repeat_height)}
    repeat_rocks = 0
    while True:
        fall_rock()
        repeat_rocks += 1
        for l in range(repeat_height):
            if world[max_height - l] != repeat_shape[l]:
                break
        else:
            break

    to_fill = MAX_ROCKS - i
    repeats = to_fill // repeat_rocks - 2
    i += repeats * repeat_rocks
    max_height += repeats * repeat_height

    world = {}
    for l in range(repeat_height):
        world[max_height - l] = repeat_shape[l]

    while i < MAX_ROCKS:
        fall_rock()

    print(max_height)


if __name__ == "__main__":
    if PART_ONE:
        one()
    else:
        two()
