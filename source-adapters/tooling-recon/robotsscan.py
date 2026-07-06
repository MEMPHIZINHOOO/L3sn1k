import requests

from wellknown_files_checker import wellknownfinder

def robotsprinter(domain):
    
    result = wellknownfinder(domain)
    endpoint = result["/robots.txt"]
    if endpoint.get("status-code")==200:
        url = f"https://{domain}/robots.txt"
        response = requests.get(url)
        final_response = response.text
        return final_response
