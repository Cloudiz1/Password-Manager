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
