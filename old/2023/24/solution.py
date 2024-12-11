import sys
from z3 import *

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "2":
        f_name = "data.txt"
    else:
        f_name = "data_test2.txt"

    storms = []
    with open(f_name, "r", encoding="utf-8") as ifile:
        for line in ifile:
            l = line.strip().split(" @ ")
            start = l[0].split(", ")
            direction = l[1].split(", ")

            storms.append(
                (
                    (int(start[0]), int(start[1]), int(start[2])),
                    (int(direction[0]), int(direction[1]), int(direction[2])),
                )
            )
    s = Solver()
    x, y, z = Ints("x y z")
    vx, vy, vz = Ints("vx vy vz")

    for i, storm in enumerate(storms):
        t = Int(f"t{i}")
        s.add(t >= 0)
        s.add(x + vx * t == storm[0][0] + storm[1][0] * t)
        s.add(y + vy * t == storm[0][1] + storm[1][1] * t)
        s.add(z + vz * t == storm[0][2] + storm[1][2] * t)

    print(s.check())
    sx = s.model()[x]
    sy = s.model()[y]
    sz = s.model()[z]

    print(f"x: {sx}")
    print(f"y: {sy}")
    print(f"z: {sz}")

    total = int(sx.as_string()) + int(sy.as_string()) + int(sz.as_string())

    print(total)
