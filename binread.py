import sys

b = []

with open(sys.argv[1], "rb") as f:
    b = f.read()

with open(sys.argv[2], "w") as f:
    f.write("\n".join(bin(x) for x in b))
