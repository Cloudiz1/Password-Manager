from SboxTuples import *

lookup = [[0 for x in range(16)] for x in range(16)]
iteration = 0
for row in range(16):
    for column in range(16):
        lookup[row][column] = hex(Sbox_inv[iteration])
        iteration += 1
            
with open("Sbox.py", "a") as f:
    f.write(str(lookup))