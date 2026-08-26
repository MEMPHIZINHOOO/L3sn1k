from curl_cffi import requests
from curl_cffi.requests.exceptions import Timeout,HTTPError, SSLError, RequestException, TooManyRedirects, ConnectionError


wellknownpaths = [
    # general disc
    "/robots.txt",
    "/sitemap.xml",
    "/sitemap_index.xml",
    "/sitemap-index.xml",
    "/humans.txt",
    "/security.txt",
    "/ads.txt",
    "/app-ads.txt",

    #security 
    "/.well-known/security.txt",
    "/.well-known/security.txt.sig",
    "/.well-known/change-password",
    "/.well-known/ssh-known-hosts",
    "/.well-known/sshfp",

    #mobile 
    "/.well-known/assetlinks.json",
    "/.well-known/apple-app-site-association",
    "/.well-known/apple-developer-merchantid-domain-association",

    #auth/oauth
    "/.well-known/openid-configuration",
    "/.well-known/oauth-authorization-server",
    "/.well-known/oauth-protected-resource",
    "/.well-known/jwks.json",
    "/.well-known/webfinger",
    "/.well-known/host-meta",
    "/.well-known/host-meta.json",
    "/.well-known/uma2-configuration",
    "/.well-known/openid-federation",
    "/.well-known/ssf-configuration",

    #mail
    "/.well-known/mta-sts.txt",
    "/.well-known/autoconfig/mail",

    #social...federation
    "/.well-known/nodeinfo",
    "/.well-known/matrix/server",
    "/.well-known/matrix/client",
    "/.well-known/nostr.json",
    "/.well-known/atproto-did",

    # cert validation
    "/.well-known/acme-challenge/",
    "/.well-known/pki-validation/",
    "/.well-known/est",
    "/.well-known/cmp",

    #api
    "/.well-known/api-catalog",
    "/.well-known/terraform.json",
    "/.well-known/ai-plugin.json",
    "/.well-known/agent-card.json",
    "/.well-known/mercure",

    #privacy
    "/.well-known/gpc.json",
    "/.well-known/dnt",
    "/.well-known/dnt-policy.txt",
    "/.well-known/traffic-advice",

    #decid and metadata
    "/.well-known/did.json",
    "/.well-known/did-configuration.json",
    "/.well-known/trust.txt",
    "/.well-known/keybase.txt",

    #supply chain(sec metadata)
    "/.well-known/sbom",
    "/.well-known/csaf",
    "/.well-known/csaf-aggregator",
]

def wellknownfinder(domain, bt="chrome"):

    resultlist={}

    url = f"https://{domain}"
     
    for i in wellknownpaths:
        
        try:
            visited_endpoint = url + i
            response = requests.get(visited_endpoint,impersonate=bt, timeout=3)
            resultlist[i]={
                
                "status-code": response.status_code,
                "content-type": response.headers.get("Content-Type"),
                "content-length": response.headers.get("Content-Length"),
                "final-url": response.url,
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
        except HTTPError:
            resultlist[i]={"error": "http error"}
        except RequestException:
            resultlist[i]={"error": "request exception"}

    return resultlist        
