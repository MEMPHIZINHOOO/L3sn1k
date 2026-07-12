from curl_cffi import requests
from requests.exceptions import Timeout, HTTPError, SSLError, RequestException, TooManyRedirects, ConnectionError

headerslist = [
    # Core security headers
    "Strict-Transport-Security",
    "Content-Security-Policy",
    "Content-Security-Policy-Report-Only",
    "X-Frame-Options",
    "X-Content-Type-Options",
    "Referrer-Policy",
    "Permissions-Policy",
    "Feature-Policy",

    #Cookies
    "Set-Cookie",

    # Cross-origin isolation/ browser security
    "Cross-Origin-Opener-Policy",
    "Cross-Origin-Resource-Policy",
    "Cross-Origin-Embedder-Policy",
    "Origin-Agent-Cluster",

    # CORS related 
    "Access-Control-Allow-Origin",
    "Access-Control-Allow-Credentials",
    "Access-Control-Allow-Methods",
    "Access-Control-Allow-Headers",
    "Access-Control-Expose-Headers",
    "Access-Control-Max-Age",
    "Vary",

    # Reporting/monitoring
    "Reporting-Endpoints",
    "Report-To",
    "NEL",
    "Expect-CT",

    # Cache / sensitive data exposure
    "Cache-Control",
    "Pragma",
    "Expires",

    # Authentication-related
    "WWW-Authenticate",
    "Proxy-Authenticate",
    "Authentication-Info",

    # Legacy / deprecated but still useful xD
    "X-XSS-Protection",
    "Public-Key-Pins",
    "Public-Key-Pins-Report-Only",
    "X-Permitted-Cross-Domain-Policies",
    "X-Download-Options",
    "X-DNS-Prefetch-Control",

    # ID and fingerprinting
    "Server",
    "X-Powered-By",
    "X-AspNet-Version",
    "X-AspNetMvc-Version",
    "X-Generator",
    "Via",

    # Misc security-relevant ._.
    "Clear-Site-Data",
    "Service-Worker-Allowed",
    "Alt-Svc",
    "Upgrade-Insecure-Requests",
]

headerslistlower = [item.lower() for item in headerslist]


def getheaders(domain, bt="chrome"):
	presentheaders={}
	try:
		url =f"https://{domain}/"
		result = requests.get(url,impersonate=bt,timeout=3)
		presentheaders["status-code"] = result.status_code
		headers = result.headers
		lowerheaders ={key.lower(): value for key, value in headers.items()}

		for i in headerslistlower:
			if i in lowerheaders:
				presentheaders[i]=lowerheaders[i]
				#print(presentheaders)

		if len(presentheaders)==1:
			print("No security headers found!")

	except Timeout:
		presentheaders["error"]="timeout"
	except SSLError: 
		presentheaders["error"]="ssl error"
	except TooManyRedirects:
		presentheaders["error"]="too many redirects"
	except ConnectionError:
		presentheaders["error"]="connection error"
	except HTTPError:
		presentheaders["error"]="http error"
	except RequestException:
		presentheaders["error"]="request exception error"

	return presentheaders


