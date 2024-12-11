from sys import argv

MAX_VALUE = 9999999999999

class Field:
    def __init__(self, height: int):
        self.height = height
        self.fields = []
        self.adjacent = []
        self.score = MAX_VALUE


def main():
    fields = []
    end = None
    start = None
    a_fields = []

    with open("data.txt" if len(argv) < 2 else argv[1]) as ifile:
        for line in ifile.readlines():
            fields.append([])
            line = line.strip()
            for c in line:
                if c == "E":
                    to_add = Field(25)
                    end = to_add
                elif c == "S":
                    to_add = Field(0)
                    start = to_add
                elif c == "a":
                    to_add = Field(0)
                    a_fields.append(to_add)
                else:
                    to_add = Field(ord(c)-97)
                fields[-1].append(to_add)

    for y in range(len(fields)):
        for x in range(len(fields[y])):
            this = fields[y][x]
            if y > 0:
                other = fields[y-1][x]
                this.adjacent.append(other)
                if other.height - 1 <= this.height:
                    this.fields.append(other)
            if x > 0:
                other = fields[y][x-1]
                this.adjacent.append(other)
                if other.height - 1 <= this.height:
                    this.fields.append(other)
            if y < len(fields) - 1:
                other = fields[y+1][x]
                this.adjacent.append(other)
                if other.height - 1 <= this.height:
                    this.fields.append(other)
            if x < len(fields[y]) - 1:
                other = fields[y][x+1]
                this.adjacent.append(other)
                if other.height - 1 <= this.height:
                    this.fields.append(other)

    to_change = [(end, 0)]
    while len(to_change) > 0:
        self, new_score = to_change.pop(0)
        if self.score <= new_score:
            continue
        self.score = new_score
        for field in self.adjacent:
            if self in field.fields:
                to_change.append((field, new_score+1))

    # one
    print(start.score)

    # two
    min_len = MAX_VALUE
    for field in a_fields:
        if field.score < min_len:
            min_len = field.score
    print(min_len)

if __name__ == "__main__":
    main()
