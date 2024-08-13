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
    
def print_bin(state):
    bin_array = [[0 for x in range(4)] for x in range(4)]
    
    for i in range(4):
        for j in range(4):
            if isinstance(state[i][j], str):

                bin_array[i][j] = bin(int(state[i][j], 16))
                
            else:
                bin_array[i][j] = bin(state[i][j])

    print_state(bin_array)