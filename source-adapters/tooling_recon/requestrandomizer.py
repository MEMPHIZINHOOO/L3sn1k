from .crtadapter import fetchcrt
from .securityheaders import getheaders
from .corschecker import corscheck
from .wellknown_files_checker import wellknownfinder
from .httpfetcher import requestit
from .techfingerprinter import techfprinter
from .robotsscan import robotsprinter
from .sitemapscan import sitemapprinter
import random

def randomizer(domain,bt="chrome"):
    results_fetchcert={}
    results_corschecker={}
    results_tech={}
    results_robots={}
    results_wkfinder={}
    results_getheader={}
    results_sitemap={}
    results_httpnormalrequest={}
    
    
    module_list=[1,2,3,4,5,6,7,8]
    random.shuffle(module_list)
    for i in module_list:
        match i:
            case 1:
                #print("CASE 1:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 01 ]                           │")
                print("│                      CRT.SH ENUMERATION                              │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Searching certificate transparency records...                       │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_fetchcert['fetchcrt_response'] = 4 #fetchcrt(domain, bt=bt) function not finished
                print(results_fetchcert)
                print("\n")

            case 2:
                #print("CASE 2:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 02 ]                           │")
                print("│                      SECURITY HEADERS                                │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Inspecting HTTP response security headers...                        │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_getheader['getheader_response'] = getheaders(domain, bt=bt)
                print(results_getheader)
                print("\n")

            case 3:
                #print("CASE 3:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 03 ]                           │")
                print("│                         CORS ANALYSIS                                │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Testing Cross-Origin Resource Sharing configuration...              │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_corschecker['corschecker_response'] = corscheck(domain,bt=bt)
                print(results_corschecker)
                print("\n")

            case 4:
                #print("CASE 4:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 04 ]                           │")
                print("│                    WELL-KNOWN DISCOVERY                              │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Searching common and .well-known resources...                       │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_wkfinder['wkfinder_response'] = wellknownfinder(domain, bt=bt)
                print(results_wkfinder)
                print("\n")

            case 5:
                #print("CASE 5:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 05 ]                           │")
                print("│                   TECHNOLOGY FINGERPRINT                             │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Identifying frameworks, servers and technologies...                 │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_tech['techs'] = techfprinter(domain, bt=bt)
                print(results_tech)
                print("\n")
            
            case 6:
                #print("CASE 6:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 06 ]                           │")
                print("│                       ROBOTS.TXT SCAN                                │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Inspecting robots.txt directives and exposed paths...               │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_robots['robots'] = robotsprinter(domain, bt=bt)
                print(results_robots)
                print("\n")

            case 7:
                #print("CASE 7:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 07 ]                           │")
                print("│                          HTTP PROBE                                  │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Sending baseline HTTP request to target...                          │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_httpnormalrequest['httpnormalrequest'] = requestit(domain, bt=bt)
                print(results_httpnormalrequest)
                print("\n")

            case 8 :
                #print("CASE 8:")
                print("╭──────────────────────────────────────────────────────────────────────╮")
                print("│                    [ L3SN1K :: MODULE 08 ]                           │")
                print("│                       SITEMAP DISCOVERY                              │")
                print("├──────────────────────────────────────────────────────────────────────┤")
                print("│  Searching and parsing sitemap resources...                          │")
                print("╰──────────────────────────────────────────────────────────────────────╯")
                results_sitemap['sitemap'] = sitemapprinter(domain, bt=bt)
                print(results_sitemap)
                print("\n")

    #return results
