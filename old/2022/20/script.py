from __future__ import annotations

TEST = False
VERBOSITY = 2


class Node:
    def __init__(self, prev: [Node, None], value: int):
        self.prev: [Node, None] = prev
        self.next: [Node, None] = None
        self.value = value

    def __str__(self):
        return str(self.value)

    def remove(self):
        prev = self.prev
        prev.next = self.next
        prev.next.prev = prev

    def insert_after(self, other: Node):
        next_n = self.next
        self.next = other
        other.prev = self
        other.next = next_n
        next_n.prev = other

    def get(self, index: int) -> Node:
        if index <= 0:
            current = self.prev
            for _ in range(-index):
                current = current.prev
            return current
        current = self
        for _ in range(index):
            current = current.next
        return current


def compute(mixes: int = 1, key: int = 1):
    nodes = []
    zero = None
    with open("data_test.txt" if TEST else "data.txt") as ifile:
        for line in ifile.readlines():
            if len(nodes) == 0:
                nodes.append(Node(None, int(line.strip()) * key))
            else:
                nodes.append(Node(nodes[-1], int(line.strip()) * key))
                nodes[-2].next = nodes[-1]
            if nodes[-1].value == 0:
                zero = nodes[-1]
        nodes[0].prev = nodes[-1]
        nodes[-1].next = nodes[0]

    for i in range(mixes):
        if VERBOSITY > 1:
            print(i)
        for node in nodes:
            to_move = node.value % (len(nodes) - 1)
            if to_move == 0:
                continue
            node.remove()
            new_prev = node.get(to_move)
            new_prev.insert_after(node)

    return nodes, zero


def print_nodes(zero):
    current = zero.next
    while current.value != 0:
        print(current)
        current = current.next
    pass


def one():
    _, zero = compute(1, 1)
    print(zero.get(1000).value + zero.get(2000).value + zero.get(3000).value)


def two():
    _, zero = compute(10, 811589153)
    print(zero.get(1000).value + zero.get(2000).value + zero.get(3000).value)


if __name__ == "__main__":
    one()
    two()
