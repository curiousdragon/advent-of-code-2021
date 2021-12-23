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

# Solution: part 1
def count_increase(data):
    """Return the number of times an element of DATA increases from its
    previous element.
    >>> data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    >>> count_increase(data)
    7
    """
    return sum([data[i] > data[i-1] for i in range(1, len(data))])

# output = str(count_increase(data))

def count_increase_window(data, window):
    """Return the number of times a WINDOW-sized window of elements of DATA
    increases from the previous window.
    >>> data = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    >>> count_increase_window(data, 3)
    5
    """
    def wsum(i):
        """The window sum."""
        return sum([data[i + k] for k in range(window)])
    return sum([wsum(i) < wsum(i+1) for i in range(len(data) - window)])

# Solution: part 2
output = str(count_increase_window(data, 3))

# Write the output
path = 'output.txt'
with open(path, 'w') as f:
    f.write(output + '\n')

