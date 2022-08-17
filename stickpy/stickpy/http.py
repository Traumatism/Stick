import requests

from requests import Response


def fetch(url: str) -> Response:
    """Fetch the response of a HTTP GET request"""
    return requests.get(url)
