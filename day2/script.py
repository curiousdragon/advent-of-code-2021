# Day 2

# Imports
import csv

# Read in the input
path = 'input.txt'
data = []
with open(path, newline='') as f:
    reader = csv.reader(f, delimiter='\n')
    for line in reader:
        split_line = line[0].split()
        data.append([split_line[0], int(split_line[1])])

# Solution
def direction(cmd):
    """COORDS: [horizontal position, depth]."""
    def forward(coords, x):
        coords[0] += x
        coords[1] += coords[2] * x
        return coords

    def down(coords, x):
        coords[2] += x
        return coords

    def up(coords, x):
        coords[2] -= x
        return coords

    commands = {
            "forward": forward,
            "down": down,
            "up": up
            }

    if cmd not in commands:
        print(f"Error: {cmd} not in commands")
        return

    return commands[cmd]

def calculate_coords(data):
    """Calculate the horizontal position and the depth
    according to the directions given in DATA.
    Return a list whose first element is the horizontal position,
    and whose second element is the depth.
    >>> data = [["forward", 5], ["down", 5], ["forward", 8],
    ...     ["up", 3], ["down", 8], ["forward", 2]]
    >>> calculate_coords(data)
    [15, 60, 10]
    """
    coords = [0, 0, 0]
    return [direction(line[0])(coords, line[1]) for line in data][-1]

coords = calculate_coords(data)
output = str(coords) + '\n' + str(coords[0] * coords[1])

# Write the output
path = 'output.txt'
with open(path, 'w') as f:
    f.write(output + '\n')

