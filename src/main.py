from inputs import *
from rounds import *
from print_state import *

test_input = "abcdefghijklmnopqrstuvwxyz"

states = create_states(test_input)
keys = generate_keys()
for state in states:
    add_round_key(state, keys[0])
# print("\nfinal state(s):")
# for state in states:
#     print_state(state)