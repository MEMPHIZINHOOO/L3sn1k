import requests
from pubdocs.pubdocsfinder import queriesforsearch
from pathlib import Path
import json

config_file = Path.home() / ".config" / "l3sn1k" /"config.json"

with open(config_file) as f:
    config=json.load(f)

api_key = config["search_api_key"]

def search(query):
    
    params = {
        "q": query
    }

    headers = {
        "Authorization" : f"Bearer" {api_key}
    }

    r = requests.get(url, params=params)

    return response.json()
