import numpy as np

visited = {0:{0: True}}
TAIL_LEN = 1


def visit(x: int, y: int):
    global visited
    if x not in visited.keys():
        visited[x] = {}
    visited[x][y] = True

def maybe_move(prev, prev_old, current):
    if np.linalg.norm(prev-current) >= 2:
        return (prev_old, True)
    return (current, False)


with open("data.txt") as ifile:
    instructions = []
    for line in ifile.readlines():
        line = line.strip()
        if line == "":
            continue
        opts = line.split(' ')
        assert len(opts) == 2

        direction = None
        match opts[0]:
            case "U":
                direction = np.array([0,1])
            case "D":
                direction = np.array([0,-1])
            case "R":
                direction = np.array([1,0])
            case "L":
                direction = np.array([-1,0])
        instructions.append((direction, int(opts[1])))

T = [np.array([0,0]) for _ in range(TAIL_LEN + 1)]

for ins in instructions:
    for _ in range(1, ins[1]):
        prev_T = T.copy()

        T[0] += ins[0]
        for i in range(TAIL_LEN):
            c = maybe_move(T[i-1], prev_T[i-1], prev_T[i])
            T[i] = c[0]
            if c[1]:
                break
        visit(T[-1][0], T[-1][1])
            

count = 0
for k_out in visited:
    for k_in in visited[k_out]:
        count += 1
print(count)
