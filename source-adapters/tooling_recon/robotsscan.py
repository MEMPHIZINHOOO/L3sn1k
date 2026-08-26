from curl_cffi import requests

from .wellknown_files_checker import wellknownfinder

def robotsprinter(domain, bt="chrome"):
    
    result = wellknownfinder(domain, bt)
    endpoint = result["/robots.txt"]
    if endpoint.get("status-code")==200:
        url = f"https://{domain}/robots.txt"
        response = requests.get(url,impersonate=bt)
        final_response = response.text
        return final_response
