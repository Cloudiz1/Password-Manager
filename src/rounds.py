from lookup import *
from print_state import *
import json
import binascii

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
            
def shift_rows(state):
    shift_rows_parent(state, "left")
    
def inverse_shift_rows(state):
    shift_rows_parent(state, "right")
    
def GF(byte, num): # Galois field multiplication
    match(num):
        case 2:
            result = byte << 1 # bitshift
            
            if byte & 0x80: 
                result ^= 0x1B # modulo
            
            return result & 0xFF # only return last 8 (I think) bits
            
        case 3:
            result = GF(byte, 2) ^ byte 
            return result

        case 9:
            result = GF(byte, 2)
            result = GF(result, 2)
            result = GF(result, 2) ^ byte
            return result
            
        case 11:
            result = GF(byte, 2)
            result = GF(result, 2) ^ byte
            result = GF(result, 2) ^ byte
            return result
        
        case 13:
            result = GF(byte, 2) ^ byte
            result = GF(result, 2)
            result = GF(result, 2) ^ byte
            return result

        case 14:
            result = GF(byte, 2) ^ byte
            result = GF(result, 2) ^ byte
            result = GF(result, 2)
            return result
    
    
def mix_column(column):
    mixed_column = [0 for x in range(4)]
    
    mixed_column[0] = GF(column[0], 2) ^ GF(column[1], 3) ^ column[2] ^ column[3]
    mixed_column[1] = column[0] ^ GF(column[1], 2) ^ GF(column[2], 3) ^ column[3]
    mixed_column[2] = column[0] ^ column[1] ^ GF(column[2], 2) ^ GF(column[3], 3)
    mixed_column[3] = GF(column[0], 3) ^ column[1] ^ column[2] ^ GF(column[3], 2)

    return mixed_column

def inv_mix_column(column):
    unmixed_column = [0 for x in range(4)]
    
    unmixed_column[0] = GF(column[0], 14) ^ GF(column[1], 11) ^ GF(column[2], 13) ^ GF(column[3], 9)
    unmixed_column[1] = GF(column[0], 9) ^ GF(column[1], 14) ^ GF(column[2], 11) ^ GF(column[3], 13)
    unmixed_column[2] = GF(column[0], 13) ^ GF(column[1], 9) ^ GF(column[2], 14) ^ GF(column[3], 11)
    unmixed_column[3] = GF(column[0], 11) ^ GF(column[1], 13) ^ GF(column[2], 9) ^ GF(column[3], 14)
    
    return unmixed_column
        
def mix_columns_parent(state, mode):
    for column in range(len(state)):
        buffer = [] # tmp column
        for row in range(len(state)):
            buffer.append(state[row][column])
            
        if mode == "mix":
            mixed_column = mix_column(buffer)
            for row, byte in enumerate(mixed_column):
                state[row][column] = byte
                
        elif mode == "inv":
            unmixed_column = inv_mix_column(buffer)
            for row, byte in enumerate(unmixed_column):
                state[row][column] = byte
            
def mix_columns(state):
    mix_columns_parent(state, "mix")
    
def inv_mix_columns(state):
    mix_columns_parent(state, "inv")

def load_key(path):
    with open(path, "r") as f:
        key_dict = json.load(f)
        key = key_dict["key"]
        print(key, len(key))
        
# mix_column([0xdb, 0x13, 0x53, 0x45])
# state = [
#     [0xdb, 0xf2, 0x01, 0xc6],
#     [0x13, 0x0a, 0x01, 0xc6],
#     [0x53, 0x22, 0x01, 0xc6],
#     [0x45, 0x5c, 0x01, 0xc6]
# ]
# mix_columns(state)
# print_hex(state)

# inv_mix_columns(state)
# print_hex(state)

# print(hex(GF(0xdf, 14)))

load_key("key.json")