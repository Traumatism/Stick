import json
import asyncio
import typing

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
    async def scan(host: str, port: int) -> typing.Tuple[str, int, bool]:
        try:
            await asyncio.wait_for(asyncio.open_connection(host, port), 3)
        except Exception:
            return (host, port, False)

        return (host, port, True)

    async def run() -> typing.List[typing.Tuple[str, int, bool]]:

        results = asyncio.gather(
            *[asyncio.create_task(scan(value, port)) for port in DBMS.keys()]
        )

        return await results

    return json.dumps(
        {
            "nodes": [
                {
                    "name": "Databases found",
                    "rows": [
                        {"key": str(port), "value": DBMS.get(port)}
                        for _, port, _ in filter(lambda k: k[2], asyncio.run(run()))
                    ],
                },
            ]
        }
    )


if __name__ == "__main__":
    import rich
    import sys

    rich.print(execute(sys.argv[1]))
