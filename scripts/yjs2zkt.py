#!/usr/bin/python3

import sys
import json

from io import StringIO

ifile = sys.argv[1]
modname = sys.argv[2]
ofile = sys.argv[3]

with open(ifile, "rt") as f:
    txt = f.read()

obj = json.loads(txt)
obj = obj["modules"][modname]

ports, cells = obj["ports"], obj["cells"]

inputs = StringIO()
outputs = StringIO()
wirings = StringIO()

for pname, v in ports.items():
    direction, wires = v["direction"], v["bits"] 
    if direction == "input":
        for i, wire in enumerate(wires):
            print(f"{wire} {pname}[{i}]", file=inputs)
    elif direction == "output":
        for i, wire in enumerate(wires):
            print(f"{wire} {pname}[{i}]", file=outputs)

for _, v in cells.items():
    g = v["type"][2:-1].lower()
    c = v["connections"]
    
    if g == "not":
        a, y = c["A"], c["Y"]
        b = a
    else:
        a, b, y = c["A"], c["B"], c["Y"]

    print(f"{g} {a[0]} {b[0]} {y[0]}", file=wirings)

parsed = StringIO()

print("inputs", file=parsed)
print(inputs.getvalue(), file=parsed)
print("outputs", file=parsed)
print(outputs.getvalue(), file=parsed)
print("wirings", file=parsed)
print(wirings.getvalue(), file=parsed, end="")

with open(ofile, "wt") as f:
    print(parsed.getvalue(), end="", file=f)
