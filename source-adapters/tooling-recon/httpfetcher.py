from curl_cffi import requests
from curl_cffi.exceptions import Timeout

def requestit(domain, bt="chrome"):    
    try:    
        url=f"https://{domain}/"
        normal_request = requests.get(url,impersonate=bt, timeout=3)
        request_response = normal_request.headers
    except Timeout:
        request_response["error"]="timeout"
        
    return request_response
