
import json

class ModuleInfos:

    name = "testmodule"
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
