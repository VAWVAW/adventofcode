import re

TEST = False


class Valve:
    def __init__(self, name: str, number: list[int], flow: int, others):
        self.name = name
        self.flow = flow
        self.others = others
        if flow > 0:
            self.number = 1 << number[0]
            number[0] += 1
        else:
            self.number = 0

    def __str__(self):
        return self.name


class State:
    def __init__(self, current: str, open_valves: int, score: int, elephant: str = None):
        self.current = current
        self.score = score
        self.open_valves = open_valves
        self.elephant = elephant


def one():
    valves = {}
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        number = [0]
        for line in ifile.readlines():
            vls = re.findall(r'[A-Z]{2}', line)
            flow = int(re.findall(r'\d+', line)[0])

            valves[vls[0]] = Valve(vls[0], number, flow, vls[1:])

    to_cal = [State("AA", 0, 0)]
    best = {}
    target_best = 0

    def is_best(state: State) -> bool:
        if state.score < target_best:
            return False

        key = (state.current, state.open_valves)
        if key not in best or best[key] < state.score:
            best[key] = state.score
            return True
        return False

    best_history = []

    for time in range(1, 31):
        best_history.append(0 if len(best) == 0 else max(best.values()))
        best_change = max(x.flow for x in valves.values()) * (30 - time)
        target_best = int(best_history[-1] - best_change)

        next_cal = []
        for state in to_cal:
            val = valves[state.current]
            if val.flow > 0 and (state.open_valves & val.number) == 0:
                n = State(
                    current=state.current,
                    open_valves=state.open_valves | val.number,
                    score=state.score + val.flow * (30 - time)
                )
                if is_best(n):
                    next_cal.append(n)
            for other in val.others:
                n = State(
                    current=other,
                    open_valves=state.open_valves,
                    score=state.score
                )
                if is_best(n):
                    next_cal.append(n)
        to_cal = next_cal
    print(max(best.values()))


def two():
    valves = {}
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        number = [0]
        for line in ifile.readlines():
            vls = re.findall(r'[A-Z]{2}', line)
            flow = int(re.findall(r'\d+', line)[0])

            valves[vls[0]] = Valve(vls[0], number, flow, vls[1:])

    to_cal = [State("AA", 0, 0, elephant="AA")]
    best = {}
    target_best = 0

    def is_best(state: State) -> bool:
        if state.score < target_best:
            return False

        key = (state.current, state.elephant, state.open_valves)
        if key not in best or best[key] < state.score:
            best[key] = state.score
            return True
        return False

    best_history = []

    for time in range(1, 27):
        best_history.append(0 if len(best) == 0 else max(best.values()))
        best_change = max(x.flow for x in valves.values()) * (26 - time)
        target_best = int(best_history[-1] - best_change)

        next_cal = []
        for state in to_cal:
            val = valves[state.current]
            el = valves[state.elephant]
            if val.flow > 0 and (state.open_valves & val.number) == 0:
                if el.flow > 0 and (state.open_valves & el.number) == 0 and val.number != el.number:
                    n = State(
                        current=state.current,
                        open_valves=state.open_valves | val.number | el.number,
                        score=state.score + (val.flow + el.flow) * (26 - time),
                        elephant=state.elephant
                    )
                    if is_best(n):
                        next_cal.append(n)
                for el_other in el.others:
                    n = State(
                        current=state.current,
                        open_valves=state.open_valves | val.number,
                        score=state.score + val.flow * (26 - time),
                        elephant=el_other
                    )
                    if is_best(n):
                        next_cal.append(n)

            for other in val.others:
                if el.flow > 0 and (state.open_valves & el.number) == 0:
                    n = State(
                        current=other,
                        open_valves=state.open_valves | el.number,
                        score=state.score + el.flow * (26 - time),
                        elephant=state.elephant
                    )
                    if is_best(n):
                        next_cal.append(n)
                for el_other in el.others:
                    n = State(
                        current=other,
                        open_valves=state.open_valves,
                        score=state.score,
                        elephant=el_other
                    )
                    if is_best(n):
                        next_cal.append(n)
        to_cal = next_cal
    print(max(best.values()))


if __name__ == "__main__":
    one()
    two()
