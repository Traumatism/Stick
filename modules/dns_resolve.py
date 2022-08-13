
import contextlib
import json
import dns.resolver


class ModuleInfos:

    name = "dns_resolve"
    target_types = ["domain"]
    author = "toast <toast@mailfence.com>"
    desc = "Resolve a domain"

    def to_json(self) -> str:
        return json.dumps({
            "name": self.name,
            "target_types": self.target_types,
            "author": self.author,
            "desc": self.desc,
            "file_path": "".join(__file__.split(".py")[:-1])
        })


def execute(value: str):

    answers_a = []
    answers_mx = []

    with contextlib.suppress(Exception):
        answers_a = dns.resolver.resolve(value, 'A')

    with contextlib.suppress(Exception):
        answers_mx = dns.resolver.resolve(value, 'MX')


    return json.dumps({
        "nodes": [
            {
                "name": "DNS resolutions (A)", 
                "rows": [
                    {"key": str(i), "value": str(r)}
                    for i, r in enumerate(answers_a)
                ]
            },
            {
                "name": "DNS resolutions (MX)", 
                "rows": [
                    {"key": str(i), "value": str(r)}
                    for i, r in enumerate(answers_a)
                ]
            },
        ]
    })


if __name__ == "__main__":
    import rich
    import sys
    rich.print(execute(sys.argv[1]))
