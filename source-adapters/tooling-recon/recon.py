
from crtadapter import fetchcrt
from securityheaders import getheaders
from corschecker import corscheck
from wellknown_files_checker import wellknownfinder
from httpfetcher import requestit
from techfingerprinter import techfprinter


def get_domain():  #gets the user input (domain)i
    return input("Enter your target domain: ")

domain=get_domain()

#fetchctr_response =fetchcrt(domain) #runs the adapter for this domain
getheader_response = getheaders(domain)
corscheck_response= corscheck(domain)
wkfinder_response=wellknownfinder(domain)
#migrate to this fetcher...later
httpnormalrequest = requestit(domain)
techs=techfprinter(domain)


#print(fetchcrt_response)
#print(getheader_response)
#print(corscheck_response)
#print(wkfinder_response)
#print(httpnormalrequest)
print(techs)







