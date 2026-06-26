
from crtadapter import fetchcrt
from securityheaders import getheaders

def get_domain():  #gets the user input (domain)
    return input("Enter your target domain: ")

domain=get_domain()
result =fetchcrt(domain) #runs the adapter for this domain
result2 = getheaders(domain)
print(result)





