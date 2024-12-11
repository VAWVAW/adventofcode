from collections import deque

TEST = False


class Wind:
    def __init__(self, x: int, y: int, direction: tuple[int, int]):
        self.x = x
        self.y = y
        self.d = direction

    def move(self, world):
        self.x += self.d[0]
        self.y += self.d[1]
        if self.x < 1:
            self.x = len(world[self.y]) - 2
        elif self.y < 1:
            self.y = len(world) - 3
        elif self.x > len(world[self.y]) - 2:
            self.x = 1
        elif self.y > len(world) - 3:
            self.y = 1
        world[self.y][self.x] = 1


def clear_world(world):
    for y in range(len(world)):
        for x in range(len(world[y])):
            if world[y][x] == 1:
                world[y][x] = 0


def print_world(world):
    for line in world:
        for c in line:
            print(c, end="")
        print()


def main():
    winds = []
    world = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for y, line in enumerate(ifile.readlines()):
            world.append([])
            for x, c in enumerate(line.strip()):
                if c == "#":
                    world[-1].append(2)
                    continue
                if c == ".":
                    world[-1].append(0)
                    continue
                world[-1].append(1)
                match c:
                    case ">":
                        winds.append(Wind(x, y, (1, 0)))
                    case "v":
                        winds.append(Wind(x, y, (0, 1)))
                    case "<":
                        winds.append(Wind(x, y, (-1, 0)))
                    case "^":
                        winds.append(Wind(x, y, (0, -1)))

    world.append([2] * len(world[0]))

    states = deque([(1, 0)])
    best = len(world) * len(world[0])
    # one
    #targets = [len(world) + len(world[0]) - 4]
    # two
    targets = [len(world) + len(world[0]) - 4, 1, len(world) + len(world[0]) - 4]
    i = 0
    run = True
    while run:
        print(f"{i}: {len(states)} states; best: {best}")
        clear_world(world)
        for wind in winds:
            wind.move(world)

        next_states = deque()
        for state in states:
            distance = abs(targets[0] - state[0] - state[1])
            if distance == 0:
                print(f"reached target at {i}")
                targets.pop(0)
                if len(targets) > 0:
                    next_states = deque([state])
                    best = abs(targets[0] - state[0] - state[1])
                    break
                print(f"solution: {i}")
                run = False
                break
            if distance < best:
                best = distance
            elif distance > best * 1.1 + 20:
                continue

            if world[state[1]][state[0]] == 0:
                n = state
                if n not in next_states:
                    next_states.append(n)
            if world[state[1]-1][state[0]] == 0:
                n = (state[0], state[1]-1)
                if n not in next_states:
                    next_states.append(n)
            if world[state[1]+1][state[0]] == 0:
                n = (state[0], state[1]+1)
                if n not in next_states:
                    next_states.append(n)
            if world[state[1]][state[0]-1] == 0:
                n = (state[0]-1, state[1])
                if n not in next_states:
                    next_states.append(n)
            if world[state[1]][state[0]+1] == 0:
                n = (state[0]+1, state[1])
                if n not in next_states:
                    next_states.append(n)

        i += 1
        states = next_states


if __name__ == "__main__":
    main()
