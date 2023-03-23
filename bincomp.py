import sys

verbose_mode = any(x == "-v" or x == "--verbose" for x in sys.argv)


def verbose_print(*string):
    if verbose_mode:
        print(*string)


a, b = [], []
equal_counter = 0

with open(sys.argv[1], "rb") as f:
    a = f.read()

with open(sys.argv[2], "rb") as f:
    b = f.read()

a_len = len(a)
b_len = len(b)

if a_len != b_len:
    print(f"Different file lengths! {a_len} != {b_len}")

for i, v in enumerate(zip(a, b)):
    va, vb = v
    if va != vb:
        verbose_print(
            f"Difference in Byte {i + 1} detected: {bin(va)} != {bin(vb)}")
    else:
        equal_counter += 1

print(f"File equality {equal_counter / a_len * 100:.2f}%")
