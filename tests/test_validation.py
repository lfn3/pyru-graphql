from gql.client import Client
from pytest import raises
from gql import gql, Client
from graphql.error import GraphQLError


def test_valid_query(graphql_client: Client):
    query = gql(
        """
    {
        add(a: 1, b: 5)
    }
    """
    )

    graphql_client.validate(query)


def test_invalid_query(graphql_client: Client):
    query = gql(
        """
    {
        badQuery(wrong: 1)
    }
    """
    )
    with raises(GraphQLError):
        graphql_client.validate(query)
