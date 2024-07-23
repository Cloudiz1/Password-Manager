from inputs import *
from rounds import *
from print_state import *

test_input = "abcdefghijklmnopqrstuvwxyz"

states = create_states(test_input)
for state in states:
    print("original state:")
    print_state(state)
    print("shifted state:")
    shift_rows(state)
    print("inverse shifted:")
    inverse_shift_rows(state)
    
# print("\nfinal state(s):")
# for state in states:
#     print_state(state)