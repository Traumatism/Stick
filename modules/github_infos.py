import json
import requests


class ModuleInfos:

    name = "github_infos"
    target_types = ["social"]
    author = "toast <toast@mailfence.com>"
    desc = "Get GitHub account informations"

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
    response = requests.get(f"https://api.github.com/users/{value.split('@')[0]}")

    if response.status_code == 404:
        return json.dumps(
            {
                "nodes": [
                    {
                        "name": "GitHub data",
                        "rows": [],
                    },
                ]
            }
        )

    json_data = response.json()
    print(json_data)

    return json.dumps(
        {
            "nodes": [
                {
                    "name": "GitHub data",
                    "rows": [
                        {"key": "Username", "value": json_data.get("login") or "n/a"},
                        {"key": "Name", "value": json_data.get("name") or "n/a"},
                        {
                            "key": "Location",
                            "value": json_data.get("location") or "n/a",
                        },
                        {
                            "key": "Twitter",
                            "value": json_data.get("twitter_username") or "n/a",
                        },
                        {"key": "Email", "value": json_data.get("email") or "n/a"},
                        {"key": "Bio", "value": json_data.get("bio") or "n/a"},
                        {"key": "Blog", "value": json_data.get("blog") or "n/a"},
                        {"key": "Company", "value": json_data.get("company") or "n/a"},
                    ],
                },
            ]
        }
    )
