import re
import numpy as np
from scipy.spatial.distance import cityblock

TEST = True


def one():
    beacons = []
    sensors = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for line in ifile.readlines():
            digit_str = re.findall(r'-?\d+', line)
            x_s, y_s, x_b, y_b = [int(x) for x in digit_str]

            s = np.array((x_s, y_s))
            b = np.array((x_b, y_b))

            distance = cityblock(s, b)
            sensors.append((s, distance))
            beacons.append(b)

    y = 10 if TEST else 2000000
    cords = {}
    to_check = [beacon[0] for beacon in beacons if beacon[1] == y]
    for sensor in sensors:
        if sensor[1] < sensor[0][1] - y:
            continue

        distance = abs(sensor[0][1] - y)
        left = sensor[0][0] - sensor[1] + distance
        right = sensor[0][0] + sensor[1] - distance

        for x in range(left, right+1):
            if x in to_check:
                continue
            cords[x] = True
    print(len(cords.keys()))


def two():
    sensors = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for line in ifile.readlines():
            digit_str = re.findall(r'-?\d+', line)
            x_s, y_s, x_b, y_b = [int(x) for x in digit_str]

            s = np.array((x_s, y_s))
            b = np.array((x_b, y_b))

            distance = cityblock(s, b)
            sensors.append((s, distance))

    for y in range(20 if TEST else 4000001):
        if y % 10000 == 0:
            print(y)
        lr = []
        for sensor in sensors:
            if sensor[1] < sensor[0][1] - y:
                continue

            distance = abs(sensor[0][1] - y)
            left = sensor[0][0] - sensor[1] + distance
            right = sensor[0][0] + sensor[1] - distance
            lr.append((left, right))
        lr.sort(reverse=True)

        for i in range(len(lr) - 1):
            e1 = lr.pop()
            e2 = lr.pop()
            if e1[1] < e2[0] - 1:
                x = e1[1] + 1
                print(f"maybe: {x}, {y}")
                if x > 20 if TEST else 4000000:
                    break
                if x < 0:
                    lr.append(e2)
                    continue
                print(f"solution: {y + x * 4000000}")
                #return

            lr.append((e1[0], max(e1[1], e2[1])))


if __name__ == "__main__":
    import time
    t0 = time.time()
    two()
    t1 = time.time()
    print(t1-t0)
