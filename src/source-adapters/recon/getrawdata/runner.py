
from crtadapter import fetchcrt

def get_domain():  #gets the user input (domain)
    return input("Enter your target domain: ")

domain=get_domain()
result =fetchcrt(domain) #runs the adapter for this domain

print(result)





