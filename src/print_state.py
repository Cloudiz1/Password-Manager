# this is temporary lol
# i refuse to learn how to use pprint
def print_state(state):
    for row in state:
        print(row)
        
    print("")
    
def print_hex(state):
    state_buffer = []
    for row in range(len(state)): 
        row_buffer = []
        
        for column in range(len(state)):
            row_buffer.append(hex(state[row][column]))
        
        state_buffer.append(row_buffer)
        
    print_state(state_buffer)
    

