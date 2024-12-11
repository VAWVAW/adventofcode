import re

TEST = False

def get(apes, name) -> int:
    if isinstance(apes[name], int):
        return apes[name]
    if apes[name] is None:
        raise Exception()
    data = apes[name].split(" ")
    one = get(apes, data[0])
    two = get(apes, data[2])
    match data[1]:
        case "+":
            return one + two
        case "-":
            return one - two
        case "*":
            return one * two
        case "/":
            return one // two
    raise Exception()

def calc(apes, name, value) -> int:
    if apes[name] is None:
        return value
    data = apes[name].split(" ")
    one = two = None
    try:
        one = get(apes, data[0])
    except Exception:
        pass
    try:
        two = get(apes, data[2])
    except Exception:
        pass
    if one is None:
        match data[1]:
            case "+":
                return calc(apes, data[0], value - two)
            case "-":
                return calc(apes, data[0], value + two)
            case "*":
                return calc(apes, data[0], value // two)
            case "/":
                return calc(apes, data[0], value * two)
    else:
        match data[1]:
            case "+":
                return calc(apes, data[2], value - one)
            case "-":
                return calc(apes, data[2], one - value)
            case "*":
                return calc(apes, data[2], value // one)
            case "/":
                return calc(apes, data[2], one // value)

    
def one():
  apes = {}
  with open("data_test.txt" if TEST else "data.txt") as ifile:
      for line in ifile.readlines():
        data = line.strip().split(": ")
        if re.match(r'^\d+$', data[1]):
             data[1] = int(data[1])
        assert data[0] not in apes
        apes[data[0]] = data[1]

  print(get(apes, "root"))

def two():
    apes = {}
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for line in ifile.readlines():
            data = line.strip().split(": ")
            if re.match(r'^\d+$', data[1]):
                 data[1] = int(data[1])
            assert data[0] not in apes
            apes[data[0]] = data[1]
    apes["humn"] = None

    data = apes["root"].split(" ")
    one = two = None
    try:
        one = get(apes, data[0])
    except Exception:
        print(calc(apes, data[0], get(apes, data[2])))
    try:
        two = get(apes, data[2])
    except Exception:
        print(calc(apes, data[0], one))

if __name__ == "__main__":
	two()
