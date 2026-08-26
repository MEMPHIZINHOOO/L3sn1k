

def queriesforsearch(domain):
        queries = [

            f'site:{domain} "@{domain}" filetype:pdf',
            f'site:{domain} "@{domain}" filetype:docx',
            f'site:{domain} "@{domain}" filetype:xlsx',
            f'site:{domain} "@{domain}" filetype:pptx',
            #add more
        ]

        return queries

