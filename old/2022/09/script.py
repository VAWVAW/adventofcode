from sys import argv
import numpy as np

visited = {0: {0: True}}
TAIL_LEN = 9


def visit(x: int, y: int):
    global visited
    if x not in visited.keys():
        visited[x] = {}
    visited[x][y] = True


def maybe_move(prev, current):
    if np.linalg.norm(prev - current) == 2:
        shift = 0.5 * (prev - current)
        shift = shift.round().astype('int32')
        return current + shift, True
    if np.linalg.norm(prev - current) > 2:
        shift = (prev - current)
        if abs(shift[0]) > 1:
            shift[0] /= 2
        if abs(shift[1]) > 1:
            shift[1] /= 2
        return current + shift, True
    return current, False


# load data
with open("data.txt" if len(argv) < 2 else argv[1]) as ifile:
    instructions = []
    for line in ifile.readlines():
        line = line.strip()
        if line == "":
            continue
        opts = line.split(' ')
        assert len(opts) == 2

        match opts[0]:
            case "U":
                direction = np.array([0, 1])
            case "D":
                direction = np.array([0, -1])
            case "R":
                direction = np.array([1, 0])
            case "L":
                direction = np.array([-1, 0])
        instructions.append((direction, int(opts[1])))

T = [np.array([0, 0]) for _ in range(TAIL_LEN + 1)]

max_head = []
# execute instructions
for ins in instructions:
    for _ in range(ins[1]):
        T[0] += ins[0]
        max_head.append(np.linalg.norm(T[0]))
        for i in range(1, TAIL_LEN + 1):
            c = maybe_move(T[i - 1], T[i])
            T[i] = c[0]
            if not c[1]:
                break
        visit(T[-1][0], T[-1][1])

# print result
count = 0
for k_out in visited:
    for k_in in visited[k_out]:
        count += 1

print(count)
