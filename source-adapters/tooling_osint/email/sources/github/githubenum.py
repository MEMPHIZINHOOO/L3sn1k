import json
import requests
from pathlib import Path

#token = os.getenv("git_token")

config_file = Path.home() / ".config" / "l3sn1k" / "config.json"

with open(config_file) as f:
    config=json.load(f)

token = config["github_token"]

def searchgit(domain):

    url_code= "https://api.github.com/search/code"
    url_commits = "https://api.github.com/search/commits"
    url_issues = "https://api.github.com/search/issues"

    params = {"q" : f"@{domain}"}

    headers={"Authorization" : f"Bearer {token}"}

    r = requests.get(url_code, params =params, headers=headers)
    r1 = requests.get(url_commits,params=params,  headers=headers)
    r2 = requests.get(url_issues,params=params,  headers=headers)

# /search/code
# /search/commits
# /search/issues

    return r.json(), r1.json(), r2.json()

