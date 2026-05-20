import requests

def fetchcrt(domain):
   url = f"https://crt.sh/?q=%25.{domain}"
   r = requests.get(url)
   return r.text


 
