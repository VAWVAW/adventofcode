import time
from collections import deque

TEST = False
PART_ONE = False
VERBOSITY = 2


class State:
    def __init__(self, blueprint):
        self.blueprint = blueprint
        self.resources = {"ore": 0, "clay": 0, "obsidian": 0, "geode": 0}
        self.robots = {"ore": 1, "clay": 0, "obsidian": 0, "geode": 0}
        self.didnt_build = []

    def mine(self):
        for robot, count in self.robots.items():
            self.resources[robot] += count

    def to_much_robots(self):
        for robot, count in self.robots.items():
            if count > self.blueprint["max_resources"][robot]:
                return False
        return True

    def produce(self, robot: str):
        if robot in self.didnt_build:
            return None
        other = State(self.blueprint)
        other.resources = self.resources.copy()
        other.robots = self.robots.copy()
        other.mine()

        costs = other.blueprint["robots"][robot]
        for resource, count in costs.items():
            other.resources[resource] -= count
            assert other.resources[resource] >= 0
        other.robots[robot] += 1
        return other

    def producible(self) -> list[str]:
        ret = []
        for robot, costs in self.blueprint["robots"].items():
            if self.robots[robot] >= self.blueprint["max_resources"][robot]:
                continue
            for resource, count in costs.items():
                if self.resources[resource] < count:
                    break
            else:
                ret.append(robot)
        return ret

    def resource_key(self):
        return self.resources["geode"], self.resources["obsidian"], self.resources["clay"], self.resources["ore"]


time1 = 0


def calculate_blueprint(blueprints, mins) -> dict:
    score = {}
    for blueprint in blueprints:
        print(f"calculate blueprint {blueprint['number']}")
        states = deque()
        states.append(State(blueprint))

        for t in range(mins - 1):
            if VERBOSITY > 1:
                print(t)
            next_states = deque()
            skipped = 0
            best = 0

            for state in states:
                global time1
                t1 = time.time()
                if state is None:
                    skipped += 1
                    time1 += time.time() - t1
                    continue

                producible = state.producible()
                for robot in producible:
                    if robot in state.didnt_build:
                        time1 += time.time() - t1
                        continue
                    next_states.append(state.produce(robot))
                    state.didnt_build.append(robot)
                state.mine()

                if len(producible) == 4:
                    time1 += time.time() - t1
                    continue

                resource_key = state.resource_key()
                if resource_key[0] < best - 2:
                    time1 += time.time() - t1
                    continue
                if resource_key[0] > best:
                    best = resource_key[0]

                time1 += time.time() - t1
                next_states.append(state)

            states = next_states
        best = 0
        for state in states:
            if state is None:
                continue
            state.mine()
            best = max(best, state.resources["geode"])
        score[blueprint["number"]] = best
    return score


def main():
    blueprints = []
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for blueprint in ifile.read().split("Blueprint "):
            if blueprint == "":
                continue
            number, blueprint_s = blueprint.strip().split(":")
            number = int(number)
            robots = {}
            max_resources = {"geode": 999}
            for robot in blueprint_s.strip(" .\n").split(". "):
                type_s, costs_s = robot.split(" robot costs ")
                type_s = type_s.replace("Each ", "")
                costs = {s.split(" ")[1]: int(s.split(" ")[0]) for s in costs_s.split(" and ")}
                robots[type_s] = costs

                for resource, count in costs.items():
                    if resource not in max_resources:
                        max_resources[resource] = count
                        continue
                    max_resources[resource] = max(max_resources[resource], count)
            blueprints.append({
                "number": number,
                "robots": robots,
                "max_resources": max_resources
            })

    if PART_ONE:
        total = 0
        for number, score in calculate_blueprint(blueprints, 24).items():
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
    print(time1)
