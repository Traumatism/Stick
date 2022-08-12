import requests
import json


class ModuleInfos:
    """ Class containing module informations """

    name = "ipinfo"
    target_types = ["ip_address"]
    author = "toast <toast@mailfence.com>"
    desc = "Lookup IP address informations via ipinfo.io"

    def to_json(self) -> str:
        return json.dumps({
            "name": self.name,
            "target_types": self.target_types,
            "author": self.author,
            "desc": self.desc,
            "file_path": "".join(__file__.split(".py")[:-1])
        })


def execute(value: str):
    json_data = requests.get(f"https://ipinfo.io/{value}/json").json()
    return json.dumps({
        "nodes": [
            {
                "name": "Network data", 
                "rows": [
                    {"key": "IP", "value": json_data["ip"]},
                    {"key": "Hostname", "value": json_data.get("hostname", "n/a")},
                    {"key": "Org", "value": json_data["org"]}
                ]
            },
            {
                "name": "Location data",
                "rows": [
                    {"key": "Country", "value": json_data["country"]},
                    {"key": "Region", "value": json_data["region"]},
                    {"key": "City", "value": json_data["city"]},
                    {"key": "Lat/Long", "value": json_data["loc"].replace(",", "/")}
                ]
            },
        ]
    })


if __name__ == "__main__":
    import rich
    import sys
    rich.print(execute(sys.argv[1]))
