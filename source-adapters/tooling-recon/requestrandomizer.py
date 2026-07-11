from crtadapter import fetchcrt
from securityheaders import getheaders
from corschecker import corscheck
from wellknown_files_checker import wellknownfinder
from httpfetcher import requestit
from techfingerprinter import techfprinter
from robotsscan import robotsprinter
import random


def randomizer(domain):
    resultados={}
    module_list=[1,2,3,4,5,6,7]
    random.shuffle(module_list)
    for i in module_list:
        match i:
            case 1:
                resultados['fetchcrt_response'] = 4 #fetchcrt(domain) function not finished
            case 2:
                resultados['getheader_response'] = getheaders(domain)
            case 3:
                resultados['corschecker_response'] = corscheck(domain)
            case 4:
                resultados['wkfinder_response'] = wellknownfinder(domain)
            case 5:
                resultados['techs'] = techfprinter(domain)
            case 6:
                resultados['robots'] = robotsprinter(domain)
            case 7:
                resultados['httpnormalrequest'] = requestit(domain)

    return resultados
