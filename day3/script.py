# Day 3

# Imports
import csv

# Read in the input
path = 'input.txt'
data = []
with open(path, newline='') as f:
    reader = csv.reader(f, delimiter='\n')
    input_data = [line[0] for line in reader]
    data = input_data

# Solution
def diagnostics(data):
    """
    Calculate the gamma rate and the epsilon rate.
    >>> data = ["00100", "11110", "10110", "10111", "10101", "01111",
    ...     "00111", "11100", "10000", "11001", "00010", "01010"]
    >>> diagnostics(data)
    [22, 9]
    """
    rates = ["", ""]
    for i in range(len(data[0])):
        count = {"0": 0, "1": 0}
        for bnum in data:
            count[bnum[i]] += 1
        if count["0"] > count["1"]:
            rates = [rates[0] + "0", rates[1] + "1"]
        else:
            rates = [rates[0] + "1", rates[1] + "0"]
    rates = [int(rates[0], base=2), int(rates[1], base=2)]
    return rates


rates = diagnostics(data)
output = str(rates) + '\n' + str(rates[0] * rates[1])

# Write the output
path = 'output.txt'
with open(path, 'w') as f:
    f.write(output + '\n')

