from print_state import *
import json

def create_state(plaintext):
    missing_length = 16 - len(plaintext)
    new_state = [[0 for x in range(4)] for x in range (4)]
    
    iteration = 0
    for column in range(4):
        for row in range(4):
            if iteration < len(plaintext):
                byte_value = ord(plaintext[iteration])              
                new_state[row][column] = format(byte_value, "02X")
                iteration += 1
            else:
                new_state[row][column] = format(missing_length, "02X")
    
    return new_state
        
states = []
def create_states(plaintext):
    if len(plaintext) <= 16:
        states.append(create_state(plaintext))
        
    else:
        first_block = plaintext[:16]
        extra_characters = plaintext[16:]
        states.append(create_state(first_block))
        create_states(extra_characters)
        
    return states
 
def create_key_state(input_list):
    new_state = [[0 for x in range(4)] for x in range (4)]
    
    iteration = 0
    for column in range(4):
        for row in range(4):
            new_state[row][column] = input_list[iteration]
            iteration += 1
            
    return new_state

def return_words_from_list(input_list):
    words = []
    buffer = []
    
    iteration = 0
    for byte in input_list:
        buffer.append(byte)
        
        iteration += 1
        
        if iteration == 4:
            words.append(buffer)
            buffer = []
            iteration = 0
            
    return words

def key_into_state(key_string):
    new_state = [[0 for x in range(4)] for x in range(4)]
    
    buffer = ""
    byte_buffer = []
    for i, character in enumerate(key_string):
        buffer += character
        
        if i % 2 != 0:
            byte_buffer.append(buffer)   
            buffer = ""
    
    iteration = 0            
    for column in range(len(new_state)):
        for row in range(len(new_state)):
            new_state[row][column] = byte_buffer[iteration]
            iteration += 1
            
    return new_state
    
def load_key(path):
    with open(path, "r") as f:
        key_dict = json.load(f)
        key_array = key_dict["key"]
            
        return key_array
    
# key_into_state(load_key("key.json"))