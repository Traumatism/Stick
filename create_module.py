# creates a new module in 'modules/'

import os
import sys
import json

CODE = """
import json

class ModuleInfos:

    name = ""
    target_types = ["ip_address"]
    author = "toast <toast@mailfence.com>"
    desc = ""

    def to_json(self) -> str:
        return json.dumps({
            "name": self.name,
            "target_types": self.target_types,
            "author": self.author,
            "desc": self.desc,
            "file_path": "".join(__file__.split(".py")[:-1])
        })


def execute(value: str):
    return json.dumps({
        "nodes": [
            {
                "name": "node_name", 
                "rows": [
                    {"key": "name", "value": "xxx"},
                    {"key": "name", "value": "xxx"}
                ]
            },
        ]
    })
"""

def create(file_name):
    with open(f"modules/{file_name}.py", "w") as f:
        f.write(CODE)

    with open("modules.json", "r+") as f:
        json_data = json.load(f)
        json_data["modules"].append(f"modules/{file_name}")
        json_data["modules"] = list(set(json_data["modules"]))

    with open("modules.json", "w") as f:
        json.dump(json_data, f, indent=4)


if len(sys.argv) < 2:
    print("You need to provide the file name (for example: hello)")
    sys.exit(1)

if not os.path.exists("modules"):
    os.mkdir("modules")


create(sys.argv[1])
