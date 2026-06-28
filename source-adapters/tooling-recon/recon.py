
from crtadapter import fetchcrt
from securityheaders import getheaders
from corschecker import corscheck

def get_domain():  #gets the user input (domain)
    return input("Enter your target domain: ")

domain=get_domain()
#result1 =fetchcrt(domain) #runs the adapter for this domain
result2 = getheaders(domain)
result3= corscheck(domain)
#print(result1)
print(result2)
print(result3)






