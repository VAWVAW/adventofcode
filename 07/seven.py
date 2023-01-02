from __future__ import annotations


class File:
    def __init__(self, parent: File, name: str):
        self._parent = parent
        self.name = name

    @property
    def size(self):
        return 0

    @property
    def parent(self):
        return self._parent


class TextFile(File):
    def __init__(self, parent: File, name: str, size: int):
        super().__init__(parent=parent, name=name)
        self._size = size

    @property
    def size(self):
        return self._size


class Directory(File):
    def __init__(self, parent: File, name: str, content=None):
        super().__init__(parent, name=name)
        if content is None:
            content = {}
        self.content = content
        self.content["."] = self
        self.content[".."] = parent

    def __getitem__(self, item):
        return self.content[item]

    def add_file(self, file: File):
        self.content[file.name] = file

    @property
    def size(self):
        return sum([file.size for (name, file) in self.content.items() if name != "." and name != ".."])


class RootDirectory(Directory):
    def __init__(self):
        super().__init__(self, "/")


if __name__ == "__main__":
    root = RootDirectory()
    pwd = root

    dirs = []

    with open("seven.txt") as ifile:
        instructions = ifile.read().split("\n")

    index = 1
    while index < len(instructions):
        ins = instructions[index]
        assert ins.startswith("$")

        opts = ins.split(" ")
        assert len(opts) > 1
        match opts[1]:
            case "cd":
                assert len(opts) > 2
                pwd = pwd[opts[2]]
                index += 1
            case "ls":
                index += 1
                while index < len(instructions) and not instructions[index].startswith("$"):
                    if instructions[index] == "":
                        index += 1
                        continue
                    info = instructions[index].split(" ")
                    assert len(info) == 2

                    if info[0] == "dir":
                        to_add = Directory(parent=pwd, name=info[1])
                        dirs.append(to_add)
                    else:
                        to_add = TextFile(parent=pwd, name=info[1], size=int(info[0]))
                    pwd.add_file(to_add)
                    index += 1

    # one
    size = 0
    for d in dirs:
        if d.size <= 100000:
            size += d.size

    print(size)

    # two
    used_space = root.size
    total_space = 70000000
    needed_space = 30000000
    free_space = total_space - used_space
    to_del = needed_space - free_space

    size_dir_to_del = used_space
    for d in dirs:
        if to_del < d.size < size_dir_to_del:
            size_dir_to_del = d.size
    print(size_dir_to_del)

    del root
