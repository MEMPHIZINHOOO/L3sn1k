import requests

corsheaders=["Access-Control-Allow-Origin","Access-Control-Allow-Credentials","Access-Control-Allow-Methods","Access-Control-Allow-Headers","Access-Control-Expose-Headers","Access-Control-Max-Age","Vary"]

corsheaderslistlower=[item.lower() for item in corsheaders]

def corscheck(domain):
    presentheaders1={}
    presentheaders2={}
    presentheaders3={}
    presentheaders4={}

    url = f"https://{domain}/"
    header1={'Origin':'https://test.com'}
    header2={'Origin':'null'}
    header3={'Origin': 'https://test.com', 'Access-Control-Request-Method': 'POST', 'Access-Control-Request-Headers': 'X-Test-Header',}
    
    response1 = requests.head(url)
    headers1 =response1.headers
    lowerheaders1={key.lower(): value for key, value in headers1.items()}

    for i in corsheaderslistlower :
        if i in lowerheaders1:
            presentheaders1[i]=lowerheaders1[i]
            print(presentheaders1)

    response2 = requests.get(url, headers=header1)
    headers2 = response2.headers
    lowerheaders2={key.lower(): value for key, value in headers2.items()}

    for i in corsheaderslistlower:
        if i in lowerheaders2:
            presentheaders2[i]=lowerheaders2[i]
    
    response3 = requests.get(url, headers=header2)
    headers3 = response3.headers
    lowerheaders3={key.lower(): value for key, value in headers3.items()}

    for i in corsheaderslistlower:
        if i in lowerheaders3:
            presentheaders3[i]=lowerheaders3[i]
    
    response4 = requests.options(url, headers=header3)
    headers4 = response4.headers
    lowerheaders4={key.lower(): value for key, value in headers4.items()}

    for i in corsheaderslistlower:
        if i in lowerheaders4:
            presentheaders4[i]=lowerheaders4[i]

    #print("Normal request:", presentheaders1)
    #print("GET with Origin https://test.com", presentheaders2)
    #print("GET with Origin null:", presentheaders3)
    #print("OPTIONS preflight:", presentheaders4)
    
    return {
         
          "normal": presentheaders1,
          "origin-test": presentheaders2,
          "null-origin":presentheaders3,
          "preflight":presentheaders4,
          
    }

    
    
