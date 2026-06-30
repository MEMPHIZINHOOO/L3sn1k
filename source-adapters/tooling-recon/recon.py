
from crtadapter import fetchcrt
from securityheaders import getheaders
from corschecker import corscheck
from wellknown_files_checker import wellknownfinder

def get_domain():  #gets the user input (domain)
    return input("Enter your target domain: ")

domain=get_domain()

#fetchctr_response =fetchcrt(domain) #runs the adapter for this domain
getheader_response = getheaders(domain)
corscheck_response= corscheck(domain)
wkfinder_response=wellknownfinder(domain)

#print(fetchcrt_response)
#print(getheader_response)
#print(corscheck_response)
print(wkfinder_response)






