from curl_cffi import requests

from .wellknown_files_checker import wellknownfinder

def sitemapprinter(domain, bt="chrome"):
        result = wellknownfinder(domain, bt=bt)
        endpoint=result["/sitemap.xml"]
        if endpoint.get("status-code")==200:
            url = f"https://{domain}/sitemap.xml"
            response=requests.get(url, impersonate=bt)
            final_response = response.txt
            return final_response

        
