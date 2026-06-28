import requests
from requests.exceptions import Timeout

corsheaders=["Access-Control-Allow-Origin","Access-Control-Allow-Credentials","Access-Control-Allow-Methods","Access-Control-Allow-Headers","Access-Control-Expose-Headers","Access-Control-Max-Age","Vary","Access-Control-Allow-Private-Network",]

corsheaderslistlower=[item.lower() for item in corsheaders]

def corscheck(domain):
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
        response_normal = requests.get(url, timeout=3)
        headers_normal =response_normal.headers
        lowerheaders_normal={key.lower(): value for key, value in headers_normal.items()}

        for i in corsheaderslistlower :
            if i in lowerheaders_normal:
                presentheaders_normal[i]=lowerheaders_normal[i]
               # print(presentheaders_normal)
    except Timeout:
        print("Timeout error on the normal request")


    #test case 1:(Origin: 'https://test.com')
    try:
        response_origintest = requests.get(url, headers=origin_test, timeout=3)
        headers_origintest = response_origintest.headers
        lowerheaders_origintest={key.lower(): value for key, value in headers_origintest.items()}
    
        for i in corsheaderslistlower:
            if i in lowerheaders_origintest:
                presentheaders_origintest[i]=lowerheaders_origintest[i]
    except Timeout:
        presentheaders_origintest["error"]="timeout"


    #test case 2:(Origin: null)
    try:
        response_originnull = requests.get(url, headers=origin_null, timeout=3)
        headers_originnull = response_originnull.headers
        lowerheaders_originnull={key.lower(): value for key, value in headers_originnull.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_originnull:
                 presentheaders_originnull[i]=lowerheaders_originnull[i]
    except Timeout:
        presentheaders_originnull["error"]="timeout"


    #testcase 3 : (OPTIONS POST)
    try:
        response_opost = requests.options(url, headers=options_post, timeout=3)
        headers_opost = response_opost.headers
        lowerheaders_opost={key.lower(): value for key, value in headers_opost.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_opost:
                presentheaders_opost[i]=lowerheaders_opost[i]
    except Timeout:
        presentheaders_opost["error"]="timeout"

    #test case 4: OPTIONS PUT
    try:
        response_oput = requests.options(url, headers=options_put, timeout=3)
        headers_oput = response_oput.headers
        lowerheaders_oput={key.lower(): value for key, value in headers_oput.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_oput:
                presentheaders_oput[i]=lowerheaders_oput[i]
    except Timeout:
        presentheaders_oput["error"]="timeout"


    #test case 5: (OPTIONS PATCH)
    try:
        response_opatch = requests.options(url, headers=options_patch, timeout=3)
        headers_opatch = response_opatch.headers
        lowerheaders_opatch={key.lower(): value for key, value in headers_opatch.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_opatch:
                presentheaders_opatch[i]=lowerheaders_opatch[i]
    except Timeout:
        presentheaders_opatch["error"]="timeout"

        
    #test case 6: (OPTIONS DELETE)
    try:
        response_odelete = requests.options(url, headers=options_delete, timeout=3)
        headers_odelete = response_odelete.headers
        lowerheaders_odelete={key.lower(): value for key, value in headers_odelete.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_odelete:
                presentheaders_odelete[i]=lowerheaders_odelete[i]
    except Timeout:
        presentheaders_odelete["error"]="timeout"

    #test case 7: origin - target as prefix
    try:
        response_evil = requests.get(url, headers=domain_evil,timeout=3 )
        headers_evil = response_evil.headers
        lowerheaders_evil={key.lower(): value for key, value in headers_evil.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_evil:
                presentheaders_evil[i]=lowerheaders_evil[i]
    except Timeout:
        presentheaders_evil["error"]="timeout"

        
    #test case 8: just http
    try:
        response_http = requests.get(url, headers=http_origin, timeout=3)
        headers_http = response_http.headers
        lowerheaders_http={key.lower(): value for key, value in headers_http.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_http:
                presentheaders_http[i]=lowerheaders_http[i]
    except Timeout:
        presentheaders_http["error"]="timeout"

        
    #test case 9:  OPTIONS PNA
    try:
        response_pna = requests.options(url, headers=options_pna, timeout=3)
        headers_pna = response_pna.headers
        lowerheaders_pna={key.lower(): value for key, value in headers_pna.items()}

        for i in corsheaderslistlower:
            if i in lowerheaders_pna:
                presentheaders_pna[i]=lowerheaders_pna[i]
    except Timeout:
        presentheaders_pna["error"]="timeout"
            
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

    
    
