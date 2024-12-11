
TEST = False


def snafu_str_to_int(s: str) -> int:
    l = len(s)
    total = 0
    for i in range(l-1, -1, -1):
        digit = 0
        match s[i]:
            case "-":
                digit = -1
            case "=":
                digit = -2
            case "1":
                digit = 1
            case "2":
                digit = 2
        total += digit * 5 ** (l - i - 1)
    return total


def int_to_snafu_str(n: int) -> str:
    ret = []
    while True:
        i = n % 5
        match i:
            case 3:
                i = -2
                ret.insert(0, "=")
            case 4:
                i = -1
                ret.insert(0, "-")
            case 0:
                ret.insert(0, "0")
            case 1:
                ret.insert(0, "1")
            case 2:
                ret.insert(0, "2")
        n += i
        n //= 5
        if i < 0:
            n += 1
        if n == 0:
            break
    return "".join(ret)


def main():
    fuels = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for line in ifile.readlines():
            fuels.append(snafu_str_to_int(line.strip()))

    print(int_to_snafu_str(sum(fuels)))


if __name__ == "__main__":
    main()
