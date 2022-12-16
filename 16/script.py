import re

TEST = True

class Valve:
    def __init__(self, flow: int = None):
        self.flow = flow
        self.other = []
        self.open = False

def main():
    valves = {}
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for line in ifile.readlines():
            vls = re.findall(r'[A-Z]{2}', line)
            flow = int(re.findall(r'\d+', line)[0])

            if vls[0] in valves.keys():
                valves[vls[0]].flow = flow
            else:
                valves[vls[0]] = Valve(flow)
            this = valves[vls[0]]
            for v in vls[1:]:
                if v not in valves.keys():
                    valves[v] = Valve()
                this.other.append(valves[v])
    start = current = valves["AA"]



if __name__ == "__main__":
    main()

