

from tooling_recon.requestrandomizer import randomizer
from tooling_osint.email.sources.github.githubenum import searchgit
from tooling_osint.engines.search_engine.search_pubdocs import search
import random

print(r"""
██╗     ██████╗ ███████╗███╗   ██╗ ██╗██╗  ██╗
██║     ╚════██╗██╔════╝████╗  ██║███║██║ ██╔╝
██║      █████╔╝███████╗██╔██╗ ██║╚██║█████╔╝
██║      ╚═══██╗╚════██║██║╚██╗██║ ██║██╔═██╗
███████╗██████╔╝███████║██║ ╚████║ ██║██║  ██╗
╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═══╝ ╚═╝╚═╝  ╚═╝
""")

def get_domain():  #gets the user input (domain)
    
    return input("Enter your target domain: ")

domain=get_domain()

#options=['chrome', 'firefox', 'safari', 'edge']
#selected=random.choice(options)
#print(f"scan stealth with signature: {selected}")
#request_sequence = randomizer(domain, bt=selected)

email_enumeration = searchgit(domain)
after_queries = search(domain)

print(osint)
#print(request_sequence)

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






