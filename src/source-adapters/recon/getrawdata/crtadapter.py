import requests
from bs4 import BeautifulSoup

def fetchcrt(domain):
   url = f"https://crt.sh/?q=%25.{domain}"
   r = requests.get(url)
   data = r.text
   soup = BeautifulSoup(data, 'html.parser')
   tables=soup.find('table')
   headers = [th.text.strip() for th in tables.find_all('th')]
   print(headers)
  





 
