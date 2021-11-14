from pytest import fixture
from gql import Client

import subprocess


@fixture(scope="session")
def graphql_schema() -> str:
    sp = subprocess.run(
        ["cargo", "run", "--example", "emit_schema"], capture_output=True
    )

    sp.check_returncode()

    return sp.stdout.decode()


@fixture(scope="session")
def graphql_client(graphql_schema) -> Client:
    return Client(schema=graphql_schema)
