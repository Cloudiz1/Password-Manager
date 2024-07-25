from inputs import *
from rounds import *
from print_state import *

test_input = "abcdefghijklmnopqrstuvwxyz"

states = create_states(test_input)
for state in states:
    mix_columns(state)
    
# print("\nfinal state(s):")
# for state in states:
#     print_state(state)