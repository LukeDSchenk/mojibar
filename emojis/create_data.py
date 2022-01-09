import json
from emoji import emojis

with open("emoji.json", "w") as fp:
    data = []
    for e in emojis:
        entry = {"name": e[1], "ch": e[0], "keywords": e[2]}
        data.append(entry)

    fp.write(json.dumps(data))

