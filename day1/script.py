# Day 1

# Imports
import csv

# Read in the input
path = 'input.txt'
data = []
with open(path, newline='') as f:
    reader = csv.reader(f, delimiter='\n')
    input_data = [int(line[0]) for line in reader]
    data = input_data

# Solution
def count_increase(data):
    """Return the number of times an element of DATA increases from its
    previous element.
    >>> data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    >>> count_increase(data)
    7
    """
    return sum([data[i] > data[i-1] for i in range(1, len(data))])

output = str(count_increase(data))

# Write the output
path = 'output.txt'
with open(path, 'w') as f:
    f.write(output + '\n')

