import os
import json
import base64

key = os.urandom(16)

json_formatted_key = {
    "key": base64.b64encode(key).hex()
}
with open("key.json", "w") as f:
    json.dump(json_formatted_key, f)