import json
import requests
import socket
import threadz
import sys


class ModuleInfos:

    name = "tld_scan"
    target_types = ["domain"]
    author = "toast <toast@mailfence.com>"
    desc = "Scan for hosts with same domain but a different TLD"

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
    def scan(domain_name: str, tld: str):
        try:
            socket.gethostbyname(f"{domain_name}.{tld}")
        except Exception:
            return (None, False)
        return (f"{domain_name}.{tld}", True)

    tld_list = list(
        map(
            lambda l: l.lower(),
            filter(
                lambda l: not l.startswith("#"),
                requests.get(
                    "https://data.iana.org/TLD/tlds-alpha-by-domain.txt"
                ).text.splitlines(),
            ),
        )
    )

    domain_name = value.split(".")[-2]

    results = threadz.gather(
        [threadz.create_task(scan, args=(domain_name, tld)) for tld in tld_list],
        concurrency=75,
    )

    return json.dumps(
        {
            "nodes": [
                {
                    "name": "Domains",
                    "rows": [
                        {"key": str(i), "value": d[0]}
                        for i, d in enumerate(
                            filter(lambda r: r[1] == True, results.values())
                        )
                    ],
                },
            ]
        }
    )


if __name__ == "__main__":
    import rich
    import sys

    rich.print(execute(sys.argv[1]))
