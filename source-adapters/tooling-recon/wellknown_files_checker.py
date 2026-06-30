import requests
from requests.exceptions import Timeout, SSLError, ConnectionError, RequestException, TooManyRedirects


wellknownpaths = [
    "/robots.txt",
    "/sitemap.xml",
    "/sitemap_index.xml",
    "/humans.txt",
    "/security.txt",

    "/.well-known/security.txt",
    "/.well-known/change-password",
    "/.well-known/assetlinks.json",
    "/.well-known/apple-app-site-association",
    "/.well-known/openid-configuration",
    "/.well-known/oauth-authorization-server",
    "/.well-known/jwks.json",
    "/.well-known/webfinger",
]

def wellknownfinder(domain):

    resultlist={}

    url = f"https://{domain}"
     
    for i in wellknownpaths:
        
        try:
            visited_endpoint = url + i
            response = requests.get(visited_endpoint, timeout=3)
            resultlist[i]={
                "status code": response.status_code,
                "content type": response.headers.get("Content-Type"),
                "content length": response.headers.get("Content-Length"),
                "final url": response.url,
                "redirected": response.url != visited_endpoint,
            }
        
        except Timeout:
            resultlist[i]={"error": "timeout"}
        except SSLError:
            resultlist[i]={"error": "ssl error"}
        except TooManyRedirects:
            resultlist[i]={"error":"too many redirects"}
        except ConnectionError:
            resultlist[i]={"error":"connection error"}
        except RequestException:
            resultlist[i]={"error": "request exception"}

    return resultlist        
