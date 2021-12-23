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

# Solution: part 1
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

# Solution: part 2
def diagnostics2(data):
    """
    Calculate the oxygen generator rating and the CO2 scrubber rating.
    >>> data = ["00100", "11110", "10110", "10111", "10101", "01111",
    ...     "00111", "11100", "10000", "11001", "00010", "01010"]
    >>> diagnostics2(data)
    [23, 10]
    """
    switches = [1, -1]
    subset = {1: data[:], -1: data[:]}
    for i in range(len(data[0])):
        # Check if we're done
        if len(subset[1]) == 1:
            if 1 in switches:
                switches.remove(1)
        if len(subset[-1]) == 1:
            if -1 in switches:
                switches.remove(-1)
        # Initialize counts
        count = {1: {"0": 0, "1": 0}, 
                -1: {"0": 0, "1": 0}}
        # switch = 1 = Oxygen: most common bit is max
        # switch = -1 = CO2: least common bit is max
        for switch in switches:
            # Compile counts for this bit
            for bnum in subset[switch]:
                count[switch][bnum[i]] += switch
            if count[switch]["0"] == count[switch]["1"]:
                subset[switch] = [n for n in subset[switch]
                        if n[i] == str(max(switch, 0))]
            elif count[switch]["0"] > count[switch]["1"]:
                subset[switch] = [n for n in subset[switch]
                        if n[i] == str(0)]
            else:
                subset[switch] = [n for n in subset[switch]
                        if n[i] == str(1)]
    return [int(subset[1][0], base=2), int(subset[-1][0], base=2)]

# rates = diagnostics(data)
rates = diagnostics2(data)
output = str(rates) + '\n' + str(rates[0] * rates[1])

# Write the output
path = 'output.txt'
with open(path, 'w') as f:
    f.write(output + '\n')

