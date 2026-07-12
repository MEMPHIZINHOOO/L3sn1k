from crtadapter import fetchcrt
from securityheaders import getheaders
from corschecker import corscheck
from wellknown_files_checker import wellknownfinder
from httpfetcher import requestit
from techfingerprinter import techfprinter
from robotsscan import robotsprinter
from sitemapscan import sitemapprinter
import random

def randomizer(domain,bt="chrome"):
    results={}
    module_list=[1,2,3,4,5,6,7,8]
    random.shuffle(module_list)
    for i in module_list:
        match i:
            case 1:
                results['fetchcrt_response'] = 4 #fetchcrt(domain, bt=bt) function not finished
            case 2:
                results['getheader_response'] = getheaders(domain, bt=bt)
            case 3:
                results['corschecker_response'] = corscheck(domain,bt=bt)
            case 4:
                results['wkfinder_response'] = wellknownfinder(domain, bt=bt)
            case 5:
                results['techs'] = techfprinter(domain, bt=bt)
            case 6:
                results['robots'] = robotsprinter(domain, bt=bt)
            case 7:
                results['httpnormalrequest'] = requestit(domain, bt=bt)
            case 8 :
                results['sitemap'] = sitemapprinter(domain, bt=bt)

    return results
