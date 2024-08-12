from inputs import *
from rounds import *
from print_state import *

test_input = "abcdefghijklmnopqrstuvwxyz"

states = create_states(test_input)
for state in states:
    print_state(state)
    sub_bytes_state = sub_bytes(state)
    print_state(sub_bytes_state)
    print_state(inverse_sub_bytes(sub_bytes_state))
# print("\nfinal state(s):")
# for state in states:
#     print_state(state)