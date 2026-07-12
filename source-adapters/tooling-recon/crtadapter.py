from curl_cffi import requests

def fetchcrt(domain, bt="chrome"):
   url = f"https://crt.sh/?q=%25.{domain}"
   r = requests.get(url,impersonate=bt)
   return r.text


 
