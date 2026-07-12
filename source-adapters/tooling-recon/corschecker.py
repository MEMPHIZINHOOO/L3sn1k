from curl_cffi import requests
from curl_cffi.requests.exceptions import Timeout, HTTPError, SSLError, RequestException, TooManyRedirects, ConnectionError
corsheaders=["Access-Control-Allow-Origin","Access-Control-Allow-Credentials","Access-Control-Allow-Methods","Access-Control-Allow-Headers","Access-Control-Expose-Headers","Access-Control-Max-Age","Vary","Access-Control-Allow-Private-Network",]

corsheaderslistlower=[item.lower() for item in corsheaders]

def corscheck(domain, bt="chrome"):
    presentheaders_normal={}
    presentheaders_origintest={}
    presentheaders_originnull={}
    presentheaders_opost={}
    presentheaders_oput={}
    presentheaders_opatch={}
    presentheaders_odelete={}
    presentheaders_pna={}
    presentheaders_http={}
    presentheaders_evil={}

    url = f"https://{domain}/"
    origin_test={'Origin':'https://test.com'}
    origin_null={'Origin':'null'}
    options_post={'Origin': 'https://test.com', 'Access-Control-Request-Method': 'POST', 'Access-Control-Request-Headers': 'X-Test-Header',}
    options_put={'Origin': 'https://test.com', 'Access-Control-Request-Method': 'PUT', 'Access-Control-Request-Headers': 'X-Test-Header',}
    options_patch={'Origin': 'https://test.com', 'Access-Control-Request-Method': 'PATCH', 'Access-Control-Request-Headers': 'X-Test-Header',}
    options_delete={'Origin': 'https://test.com', 'Access-Control-Request-Method': 'DELETE', 'Access-Control-Request-Headers': 'X-Test-Header',}
    options_pna={'Origin':'https://test.com', 'Access-Control-Request-Method':'GET','Access-Control-Request-Private-Network': 'true' }
    domain_evil={'Origin': f'https://{domain}.evil.test'}
    http_origin={'Origin':'http://test.com'}
    

    #normal request
    try:
        response_normal = requests.get(url,impersonate=bt, timeout=3)
        presentheaders_normal["status-code"]=response_normal.status_code
        headers_normal =response_normal.headers
        lowerheaders_normal={key.lower(): value for key, value in headers_normal.items()}

        for i in corsheaderslistlower :
            if i in lowerheaders_normal:
                presentheaders_normal[i]=lowerheaders_normal[i]
               # print(presentheaders_normal)
    except Timeout:
        presentheaders_normal["error"]="timeout"
    except SSLError:
        presentheaders_normal["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_normal["error"]="too many redirects"
    except ConnectionError:
        presentheaders_normal["error"]="connection error"
    except HTTPError:
        presentheaders_normal["error"]="http error"
    except RequestException:
        presentheaders_normal["error"]="request exception error"

        
    #test case 1:(Origin: 'https://test.com')
    try:
        response_origintest = requests.get(url,impersonate=bt, headers=origin_test, timeout=3)
        presentheaders_origintest["status-code"]= response_origintest.status_code
        headers_origintest = response_origintest.headers
        lowerheaders_origintest={key.lower(): value for key, value in headers_origintest.items()}
    
        for i in corsheaderslistlower:
            if i in lowerheaders_origintest:
                presentheaders_origintest[i]=lowerheaders_origintest[i]
    except Timeout:
        presentheaders_origintest["error"]="timeout"
    except SSLError:
        presentheaders_origintest["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_origintest["error"]="too many redirects"
    except ConnectionError:
        presentheaders_origintest["error"]="connection error"
    except HTTPError:
        presentheaders_origintest["error"]="http error"
    except RequestException:
        presentheaders_origintest["error"]="request exception error"


    #test case 2:(Origin: null)
    try:
        response_originnull = requests.get(url,impersonate=bt, headers=origin_null, timeout=3)
        presentheaders_originnull["status-code"] = response_originnull.status_code
        headers_originnull = response_originnull.headers
        lowerheaders_originnull={key.lower(): value for key, value in headers_originnull.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_originnull:
                 presentheaders_originnull[i]=lowerheaders_originnull[i]
    except Timeout:
        presentheaders_originnull["error"]="timeout"
    except SSLError:
        presentheaders_originnull["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_originnull["error"]="too many redirects"
    except ConnectionError:
        presentheaders_originnull["error"]="connection error"
    except HTTPError:
        presentheaders_originnull["error"]="http error"
    except RequestException:
        presentheaders_originnull["error"]="request exception error"


    #testcase 3 : (OPTIONS POST)
    try:
        response_opost = requests.options(url,impersonate=bt,headers=options_post, timeout=3)
        presentheaders_opost["status-code"]= response_opost.status_code
        headers_opost = response_opost.headers
        lowerheaders_opost={key.lower(): value for key, value in headers_opost.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_opost:
                presentheaders_opost[i]=lowerheaders_opost[i]
    except Timeout:
        presentheaders_opost["error"]="timeout"
    except SSLError:
        presentheaders_opost["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_opost["error"]="too many redirects"
    except ConnectionError:
        presentheaders_opost["error"]="connection error"
    except HTTPError:
        presentheaders_opost["error"]="http error"
    except RequestException:
        presentheaders_opost["error"]="request exception error"

    
    #test case 4: OPTIONS PUT
    try:
        response_oput = requests.options(url,impersonate=bt, headers=options_put, timeout=3)
        presentheaders_oput["status-code"]= response_oput.status_code
        headers_oput = response_oput.headers
        lowerheaders_oput={key.lower(): value for key, value in headers_oput.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_oput:
                presentheaders_oput[i]=lowerheaders_oput[i]
    except Timeout:
        presentheaders_oput["error"]="timeout"
    except SSLError:
        presentheaders_oput["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_oput["error"]="too many redirects"
    except ConnectionError:
        presentheaders_oput["error"]="connection error"
    except HTTPError:
        presentheaders_oput["error"]="http error"
    except RequestException:
        presentheaders_oput["error"]="request exception error"

        
    #test case 5: (OPTIONS PATCH)
    try:
        response_opatch = requests.options(url,impersonate=bt, headers=options_patch, timeout=3)
        presentheaders_opatch["status-code"]=response_opatch.status_code
        headers_opatch = response_opatch.headers
        lowerheaders_opatch={key.lower(): value for key, value in headers_opatch.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_opatch:
                presentheaders_opatch[i]=lowerheaders_opatch[i]
    except Timeout:
        presentheaders_opatch["error"]="timeout"
    except SSLError:
        presentheaders_opatch["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_opatch["error"]="too many redirects"
    except ConnectionError:
        presentheaders_opatch["error"]="connection error"
    except HTTPError:
        presentheaders_opatch["error"]="http error"
    except RequestException:
        presentheaders_opatch["error"]="request exception error"

        
    #test case 6: (OPTIONS DELETE)
    try:
        response_odelete = requests.options(url,impersonate=bt, headers=options_delete, timeout=3)
        presentheaders_odelete["status-code"]= response_odelete.status_code
        headers_odelete = response_odelete.headers
        lowerheaders_odelete={key.lower(): value for key, value in headers_odelete.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_odelete:
                presentheaders_odelete[i]=lowerheaders_odelete[i]
    except Timeout:
        presentheaders_odelete["error"]="timeout"
    except SSLError:
        presentheaders_odelete["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_odelete["error"]="too many redirects"
    except ConnectionError:
        presentheaders_odelete["error"]="connection error"
    except HTTPError:
        presentheaders_odelete["error"]="http error"
    except RequestException:
        presentheaders_odelete["error"]="request exception error"
    

    #test case 7: origin - target as prefix
    try:
        response_evil = requests.get(url,impersonate=bt, headers=domain_evil,timeout=3 )
        presentheaders_evil["status-code"]=response_evil.status_code
        headers_evil = response_evil.headers
        lowerheaders_evil={key.lower(): value for key, value in headers_evil.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_evil:
                presentheaders_evil[i]=lowerheaders_evil[i]
    except Timeout:
        presentheaders_evil["error"]="timeout"
    except SSLError:
        presentheaders_evil["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_evil["error"]="too many redirects"
    except ConnectionError:
        presentheaders_evil["error"]="connection error"
    except HTTPError:
        presentheaders_evil["error"]="http error"
    except RequestException:
        presentheaders_evil["error"]="request exception error"

        
    #test case 8: just http
    try:
        response_http = requests.get(url,impersonate=bt, headers=http_origin, timeout=3)
        presentheaders_http["status-code"]=response_http.status_code
        headers_http = response_http.headers
        lowerheaders_http={key.lower(): value for key, value in headers_http.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_http:
                presentheaders_http[i]=lowerheaders_http[i]
    except Timeout:
        presentheaders_http["error"]="timeout"
    except SSLError:
        presentheaders_http["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_http["error"]="too many redirects"
    except ConnectionError:
        presentheaders_http["error"]="connection error"
    except HTTPError:
        presentheaders_http["error"]="http error"
    except RequestException:
        presentheaders_http["error"]="request exception error"

        
    #test case 9:  OPTIONS PNA
    try:
        response_pna = requests.options(url,impersonate=bt, headers=options_pna, timeout=3)
        presentheaders_pna["status-code"]=response_pna.status_code
        headers_pna = response_pna.headers
        lowerheaders_pna={key.lower(): value for key, value in headers_pna.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_pna:
                presentheaders_pna[i]=lowerheaders_pna[i]
    except Timeout:
        presentheaders_pna["error"]="timeout"
    except SSLError:
        presentheaders_pna["error"]="ssl error"
    except TooManyRedirects:
        presentheaders_pna["error"]="too many redirects"
    except ConnectionError:
        presentheaders_pna["error"]="connection error"
    except HTTPError:
        presentheaders_pna["error"]="http error"
    except RequestException:
        presentheaders_pna["error"]="request exception error"
        
    #print("Normal request:", presentheaders1)
    #print("GET with Origin https://test.com", presentheaders2)
    #print("GET with Origin null:", presentheaders3)
    #print("OPTIONS preflight:", presentheaders4)
    
    return {
         
          "normal": presentheaders_normal,
          "origin-test": presentheaders_origintest,
          "origin-null":presentheaders_originnull,
          "pna-preflight":presentheaders_pna,
          "evil-origin":presentheaders_evil,
          "http-origin":presentheaders_http,
          "options-post":presentheaders_opost,
          "options-put":presentheaders_oput,
          "options-patch":presentheaders_opatch,
          "options-delete":presentheaders_odelete,
          
    }

    
    
