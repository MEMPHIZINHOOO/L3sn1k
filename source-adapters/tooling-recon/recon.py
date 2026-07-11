from requestrandomizer import randomizer

def get_domain():  #gets the user input (domain)i
    return input("Enter your target domain: ")

domain=get_domain()

request_sequence = randomizer(domain)

print(request_sequence)
#fetchctr_response =fetchcrt(domain) #runs the adapter for this domain
#getheader_response = getheaders(domain)
#corscheck_response= corscheck(domain)
#wkfinder_response=wellknownfinder(domain)
#migrate to this fetcher...later
#httpnormalrequest = requestit(domain)
#techs=techfprinter(domain)
#robots = robotsprinter(domain)

#print(fetchcrt_response)
#print(getheader_response)
#print(corscheck_response)
#print(wkfinder_response)
#print(httpnormalrequest)
#print(techs)
#print(robots)






