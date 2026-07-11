from crtadapter import fetchcrt
from securityheaders import getheaders
from corschecker import corscheck
from wellknown_files_checker import wellknownfinder
from httpfetcher import requestit
from techfingerprinter import techfprinter
from robotsscan import robotsprinter
import random
import requests

def uas():
    url = "https://raw.githubusercontent.com/projectdiscovery/useragent/main/useragent_data.json"

    response = requests.get(url, timeout=3)
    result = response.json()

    uas=[]

    for i in result:
        uas.extend(result)

all_uas = uas()

def randomizer(domain,uas_list):
    resultados={}
    module_list=[1,2,3,4,5,6,7]
    random.shuffle(module_list)
    for i in module_list:
        i_ua=random.choice(all_uas)
        match i:
            case 1:
                resultados['fetchcrt_response'] = 4 #fetchcrt(domain, user_agent=i_ua) function not finished
            case 2:
                resultados['getheader_response'] = getheaders(domain, user_agent=i_ua)
            case 3:
                resultados['corschecker_response'] = corscheck(domain, user_agent = i_ua)
            case 4:
                resultados['wkfinder_response'] = wellknownfinder(domain, user_agent = i_ua)
            case 5:
                resultados['techs'] = techfprinter(domain, user_agent = i_ua)
            case 6:
                resultados['robots'] = robotsprinter(domain, user_agent = i_ua)
            case 7:
                resultados['httpnormalrequest'] = requestit(domain, user_agent = i_ua)

    return resultados
