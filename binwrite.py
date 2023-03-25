import sys

c = ord

b = bytes([0b1, c("H"), c("e"), c("l"), c("o"),
           0b10, c("W"), c("o"), c("l"), c("d"),
           0b0,
           0b1,
           0b0, c(","), c(" "), c("\n"), c("\n"),
           0b10,
           0b0, c("\n"),
           ])

with open(sys.argv[1], "wb") as f:
    f.write(b)
