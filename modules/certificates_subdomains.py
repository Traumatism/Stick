import json
import re

from stickpy.http import fetch
from stickpy.regex import SUBDOMAIN_REGEX


class ModuleInfos:

    name = "certificates_subdomains"
    target_types = ["domain"]
    author = "toast <toast@mailfence.com>"
    desc = "Check crt.sh for known SSL certificates and extract subdomains"

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
    json_data = fetch(f"https://crt.sh/?dNSName={value}&output=json").json()

    data = " ".join(
        map(lambda o: " ".join((o["common_name"], o["name_value"])), json_data)
    )

    subdomains = re.compile(SUBDOMAIN_REGEX % value).findall(data)

    return json.dumps(
        {
            "nodes": [
                {
                    "name": "Subdomains",
                    "rows": [
                        {"key": str(i), "value": s}
                        for i, s in enumerate(set(subdomains))
                    ],
                },
            ]
        }
    )


if __name__ == "__main__":
    import sys

    print(execute(sys.argv[1]))
