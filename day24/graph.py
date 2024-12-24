# Helper code to make a graph in graphviz
# Stole from https://gist.github.com/HexTree/815a804ccd6a0e269372a73d3177636c
# To run:
# python3 -m venv v
# v/bin/pip install graphviz
# v/bin/python graph.py
import graphviz
import re

init_wires = {}
init_gates = set()
with open('input', 'r') as f:
    pattern1 = r'(\w\w\w): (\d)'
    pattern2 = r'(\w\w\w) (\D+) (\w\w\w) -> (\w\w\w)'
    for line in f.readlines():
        if re.match(pattern1, line.strip()):
            wire, val = re.match(pattern1, line.strip()).groups()
            init_wires[wire] = int(val)
        elif re.match(pattern2, line.strip()):
            wire1, op, wire2, wire3 = re.match(pattern2, line.strip()).groups()
            init_gates.add((wire1, wire2, op, wire3))
            for w in (wire1, wire2, wire3):
                if w not in init_wires:
                    init_wires[w] = None
num_x_bits = sum(w[0] == 'x' for w in init_wires)
num_y_bits = sum(w[0] == 'y' for w in init_wires)
num_z_bits = sum(w[0] == 'z' for w in init_wires)

dot = graphviz.Digraph()
for w in init_wires:
    dot.node(w)
for gate in init_gates:
    wire1, wire2, op, wire3 = gate
    label = '{} {} {}'.format(wire1, op, wire2)
    dot.node(label)
    dot.edge(wire1, label)
    dot.edge(wire2, label)
    dot.edge(label, wire3)
dot.render('graph.gv')