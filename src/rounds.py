from Sbox import *
from print_state import *

def parent_sub_bytes(state, lookup):
    for row in range(len(state)):
        for column in range(len(state)):
            curr_byte = state[row][column]
            
            curr_byte = str(format(int(curr_byte, 16), "08b")) # hex to iterable bits
            
            first_nibble = int(curr_byte[:4], 2)
            last_nibble = int(curr_byte[4:], 2)
            
            sub_byte = lookup[first_nibble][last_nibble] 
            state[row][column] = sub_byte
            
    return state

def sub_bytes(state):
    return parent_sub_bytes(state, Sbox)

def inverse_sub_bytes(state):
    return parent_sub_bytes(state, Sbox_inv)
        
def shift_rows_parent(state, direction):
    for shift_count, row in enumerate(state):
        buffer = [0 for x in range(len(row))]
        
        for iteration, byte in enumerate(row):
            if direction == "left":
                buffer[iteration - shift_count] = byte
                
            elif direction == "right":
                pointer = iteration + shift_count
                if pointer >= 4:
                    pointer -= 4
                buffer[pointer] = byte
            
        state[shift_count] = buffer
            
    print_state(state)
            
def shift_rows(state):
    shift_rows_parent(state, "left")
    
def inverse_shift_rows(state):
    shift_rows_parent(state, "right")