from curl_cffi import requests


def requestit(domain, bt="chrome"):    
    try:    
        url=f"https://{domain}/"
        normal_request = requests.get(url,impersonate=bt, timeout=3)
        request_response = normal_request.headers
    except Exception as e:
        if "timeout" in str(e).lower():
            request_response["error"]="timeout"
        else:
            request_response["error"]=f"failed: {e}"
    return request_response
