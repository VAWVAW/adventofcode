def get_first_n_dist(s:str, n: int):
    count = 0
    last = []

    for c in list(s):
        last.append(c)
        count += 1
        if len(last) > n:
            last.pop(0)
        if len(last) < n:
            continue
        
        doBreak = True
        for cl in last:
            if last.count(cl) > 1:
                doBreak = False
                break
        if doBreak:
            return count


with open("six.txt") as ifile:
    s = ifile.read()

print(get_first_n_dist(s, 4))
print(get_first_n_dist(s, 14))
