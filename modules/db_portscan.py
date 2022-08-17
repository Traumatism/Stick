import json
import socket
import typing

from stickpy.threading import gather

DBMS = {
    3306: "MySQL",
    5432: "PostgreSQL",
    6379: "Redis",
    8086: "InnoDB",
    27017: "MongoDB",
}


class ModuleInfos:

    name = "db_portscan"
    target_types = ["ip_address"]
    author = "toast <toast@mailfence.com>"
    desc = "Scan for open ports with common database engines"

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
    def scan(host: str, port: int) -> typing.Tuple[int, bool]:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.settimeout(3)
        try:
            s.connect((host, port))
        except Exception:
            return (port, False)

        return (port, True)

    results = gather(
        [(scan, (value, port), {}) for port in DBMS.keys()],
        concurrency=5,
    )

    return json.dumps(
        {
            "nodes": [
                {
                    "name": "Databases servers found",
                    "rows": [
                        {"key": str(i), "value": f"{d[0]} ({DBMS.get(d[0])})"}
                        for i, d in enumerate(
                            filter(lambda r: r[1] == True, list(results.values()))
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
