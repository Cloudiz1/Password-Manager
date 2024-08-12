import json
import random

buffer = ""
for num in range(16):
    random_byte = random.randint(0, 255)
    formatted_hex = f"{random_byte:02x}"
    buffer += formatted_hex

    

json_formatted_key = {
    "key": buffer
}

with open("key.json", "w") as f:
    json.dump(json_formatted_key, f)