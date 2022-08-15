import requests
import json
import re


class ModuleInfos:

    name = "wbu_subdomains"
    target_types = ["domain"]
    author = "toast <toast@mailfence.com>"
    desc = "Look for subdomains using WayBackURL"

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
        f"https://web.archive.org/cdx/search/cdx?url=*.{value}/*&output=json&collapse=urlkey"
    ).text

    subdomains = re.compile(r"([a-z0-9\-.]{1,}\.%s)" % value).findall(response)

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
