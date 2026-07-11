import requests
from requests.exceptions import Timeout

def requestit(domain, user_agent=None):
    headers={}
    if user_agent:
        headers['User-Agent']=user_agent    
    try:    
        url=f"https://{domain}/"
        normal_request = requests.get(url,headers=headers, timeout=3)
        request_response = normal_request.headers
    except Timeout:
        request_response["error"]="timeout"
        
    return request_response
