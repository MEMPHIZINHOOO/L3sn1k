import requests
from requests.exceptions import Timeout


def requestit(domain):
    try:
    
        url=f"https://{domain}/"
        normal_request = requests.get(url, timeout=3)
        request_response = normal_request.headers
    except Timeout:
        request_response["error"]="timeout"
        
    return request_response
