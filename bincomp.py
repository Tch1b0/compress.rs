import sys

a, b = [], []

with open(sys.argv[1], "rb") as f:
    a = f.read()

with open(sys.argv[2], "rb") as f:
    b = f.read()

for i, v in enumerate(zip(a, b)):
    va, vb = v
    if va != vb:
        print(f"Difference in Byte {i + 1} detected: {bin(va)} != {bin(vb)}")
