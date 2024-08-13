from lookup import *
from print_state import *
import json
from inputs import *

def parent_sub_bytes(state, lookup):
    for row in range(len(state)):
        for column in range(len(state)):
            curr_byte = state[row][column]
            
            sub_byte = lookup[int(curr_byte, 16)]
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
    
def list_to_hex(input_list):
    try:
        for iteration, byte in enumerate(input_list):
            input_list[iteration] = int(byte, 16)
    except:
        pass
    
    buffer = ""
    for element in input_list:
        buffer += f"{element:02x}"
        
    return buffer
    
def find_first_word(first_word, last_word, rcon): # find first byte of new word with last byte of previous word
    rot_word = last_word[1:4] + [last_word[0]]
    
    sub_word = []
    
    for word in rot_word:
        sub_word.append(int(Sbox[word], 16))
        
    sub_word[0] ^= rcon
    
    output_word = []
    for i in range(len(sub_word)):
        output_word.append(sub_word[i] ^ first_word[i])
        
    return output_word

def find_next_word(word_1, word_2): # words are inputted as arrays, should return as arrays
    buffer = []
    for i in range(len(word_1)):
        buffer.append(word_1[i] ^ word_2[i])
        
    # print("input 1: " + list_to_int(word_1) + " input 2: " + list_to_int(word_2) + " output: " + list_to_int(buffer))
        
    return buffer
    
def generate_keys():
    key_array = []
    with open("key.json", "r") as f:
        key_dict = json.load(f)
        
        buffer = ""
        for i, character in enumerate(key_dict["key"]):
            buffer += character
            
            if i % 2 != 0:
                key_array.append(int(buffer, 16))   
                buffer = ""
    
    words = return_words_from_list(key_array) # populates the list with the first key
    
    for i in range(4, 44): # 4*10 rounds; start at 4 to account for the four words already in words
        if i % 4 == 0:
            words.append(find_first_word(words[i-4], words[i-1], rcon[(i - 4)//4]))
            
        else:
            words.append(find_next_word(words[i-1], words[i-4]))
            
            
    for i in range(len(words)):
        words[i] = list_to_hex(words[i])
        
    buffer = ""
    keys = []
    for i, word in enumerate(words):
        buffer += word
        
        if (i + 1) % 4 == 0:
            keys.append(buffer)
            buffer = ""
            
    for i, key in enumerate(keys):
        keys[i] = key_into_state(key)
        
    return keys
        
    # for key in keys:
    #     print_state(key)        
    # for i, word in enumerate(words):
    #     print(list_to_int(word), i)
            
        # key_state = create_key_state(key_array)
        

        
            
        # print(byte_array)
        # key = create_state(key_hex)a
        # print_state(key)
        
def add_round_key(input_state, key_state):
    new_state = [[0 for x in range(4)] for x in range(4)]
    for i in range(4):
        for j in range(4):
            new_state[i][j] = int(input_state[i][j], 16) ^ int(key_state[i][j], 16)
    
    # print("input state:")
    # print_bin(input_state)
    
    # print("key_state:")
    # print_bin(key_state)
    
    # print("output state")
    # print_bin(new_state)
        
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