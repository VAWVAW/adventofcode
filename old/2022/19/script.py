import time
from collections import deque

TEST = False
PART_ONE = False
VERBOSITY = 1

TYPES = {
    "ore": 0,
    "clay": 1,
    "obsidian": 2,
    "geode": 3,
}


class State:
    def __init__(self, blueprint, resources, robots):
        self.blueprint = blueprint
        self.resources = resources
        self.robots = robots
        self.didnt_build = []

    def mine(self):
        self.resources = tuple(old + robot for old, robot in zip(self.resources, self.robots))

    def to_much_robots(self):
        return all(count > max_needed for count, max_needed in zip(self.resources, self.blueprint["max_resources"]))

    def produce(self, robot: int):
        if robot in self.didnt_build:
            return None

        other = State(
            self.blueprint,
            tuple(count - cost for count, cost in zip(self.resources, self.blueprint["robots"][robot])),
            self.robots
        )
        other.mine()

        other.robots = tuple(count+1 if t == robot else count for t, count in enumerate(self.robots))
        return other

    def producible(self) -> list[int]:
        ret = []
        for robot, costs in enumerate(self.blueprint["robots"]):
            if self.robots[robot] >= self.blueprint["max_resources"][robot]:
                continue
            for resource, count in enumerate(costs):
                if self.resources[resource] < count:
                    break
            else:
                ret.append(robot)
        return ret


#time1 = 0


def calculate_blueprint(blueprints, mins) -> dict:
    score = {}
    for blueprint in blueprints:
        if VERBOSITY > 0:
            print(f"calculate blueprint {blueprint['number']}")
        states = deque()
        states.append(State(blueprint, (0, 0, 0, 0), (1, 0, 0, 0)))

        for t in range(mins - 1):
            if VERBOSITY > 1:
                print(t)
            next_states = deque()
            skipped = 0
            best = 0

            for state in states:
                #global time1
                #t1 = time.time()
                if state is None:
                    skipped += 1
                    #time1 += time.time() - t1
                    continue

                producible = state.producible()
                try:
                    if t == mins - 3:
                        producible.remove(1)
                    elif t == mins - 2:
                        producible.remove(0)
                        producible.remove(1)
                        producible.remove(2)
                except ValueError:
                    pass
                for robot in producible:
                    if robot in state.didnt_build:
                        continue
                    next_states.append(state.produce(robot))
                    state.didnt_build.append(robot)
                state.mine()

                if len(producible) == 4:
                    #time1 += time.time() - t1
                    continue
                if len(state.didnt_build) == 4:
                    #time1 += time.time() - t1
                    continue

                if state.resources[3] < best - 2:
                    #time1 += time.time() - t1
                    continue
                if t == mins - 2 and state.resources[3] < best - 1:
                    continue
                if state.resources[3] > best:
                    best = state.resources[0]

                #time1 += time.time() - t1
                next_states.append(state)

            states = next_states
        best = 0
        for state in states:
            if state is None:
                continue
            state.mine()
            best = max(best, state.resources[TYPES["geode"]])
        score[blueprint["number"]] = best
        print(score)
    return score


def main():
    def list_to_tuple(l: list) -> tuple:
        return tuple(x for x in l)

    blueprints = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for blueprint in ifile.read().split("Blueprint "):
            if blueprint == "":
                continue
            number, blueprint_s = blueprint.strip().split(":")
            number = int(number)
            robots = [None, None, None, None]
            max_resources = [0, 0, 0, 999]
            for robot in blueprint_s.strip(" .\n").split(". "):
                type_s, costs_s = robot.split(" robot costs ")
                type_s = type_s.replace("Each ", "")
                costs = [0, 0, 0, 0]
                for s in costs_s.split(" and "):
                    t = s.split(" ")
                    costs[TYPES[t[1]]] = int(t[0])
                robots[TYPES[type_s]] = costs

                for resource, count in enumerate(costs):
                    max_resources[resource] = max(max_resources[resource], count)
            blueprints.append({
                "number": number,
                "robots": list_to_tuple(robots),
                "max_resources": list_to_tuple(max_resources)
            })

    if PART_ONE:
        total = 0
        for number, score in calculate_blueprint(blueprints, 24).items():
            if VERBOSITY > 0:
                print(f"{number}: {score}")
            total += score * number
        print(total)
    else:
        total = 1
        for _, score in calculate_blueprint(blueprints[:3], 32).items():
            total *= score
        print(total)


if __name__ == "__main__":
    t0 = time.time()
    main()
    print(time.time() - t0)
    #print(time1)
