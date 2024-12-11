from sys import argv
from functools import cmp_to_key

def compare(l, r):
    """
    return Option<Bool>
    True: right order
    False: wrong order
    None: not determinable
    """
    if isinstance(l, int):
        if isinstance(r, int):
            # two ints
            if l == r:
                return None
            return l < r
        l = [l]
    elif isinstance(r, int):
        r = [r]
    # two lists
    assert isinstance(l, list)
    assert isinstance(r, list)
    i = 0
    while i < len(l) and i < len(r):
        res = compare(l[i], r[i])
        if res is not None:
            return res
        i += 1
    if len(l) == len(r):
        return None
    return len(l) < len(r)


def wrap_compare(item1, item2) -> int:
    res = compare(item1, item2)
    if res is None:
        return 0
    return -1 if res else 1


def one(s_in: str):
    pairs = s_in.split("\n\n")
    count = 0
    for i in range(len(pairs)):
        l, r = pairs[i].split("\n")
        l = eval(l)
        r = eval(r)
        res = compare(l, r)

        assert res is not None
        if res:
            count += 1 + i
    print(count)


def two(s_in: str):
    packets = [eval(packet_s) for packet_s in s_in.replace("\n\n", "\n").split("\n")]
    packets.append([[2]])
    packets.append([[6]])

    packets = sorted(packets, key=cmp_to_key(wrap_compare))

    index1 = packets.index([[2]])+1
    index2 = packets.index([[6]])+1
    print(index1*index2)


def main():
    with open("data.txt" if len(argv) < 2 else argv[1]) as ifile:
        #one(ifile.read().strip())
        two(ifile.read().strip())


if __name__ == "__main__":
    main()
