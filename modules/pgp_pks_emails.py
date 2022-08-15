import re
import json
import requests


class ModuleInfos:

    name = "pgp_pks_emails"
    target_types = ["domain"]
    author = "toast <toast@mailfence.com>"
    desc = "Get emails from PGP"

    def to_json(self) -> str:
        return json.dumps(
            {
                "name": self.name,
                "target_types": self.target_types,
                "author": self.author,
                "desc": self.desc,
                "file_path": "".join(__file__.split(".py")[:-1]),
            }
        )


def execute(value: str):
    response = requests.get(
        f"https://keyserver.ubuntu.com/pks/lookup?search={value}&op=index"
    )

    emails = re.findall(r"&lt;(.*)&gt;<\/span>", response.text)

    return json.dumps(
        {
            "nodes": [
                {
                    "name": "Emails",
                    "rows": [
                        {"key": str(i), "value": e} for i, e in enumerate(set(emails))
                    ],
                },
            ]
        }
    )
