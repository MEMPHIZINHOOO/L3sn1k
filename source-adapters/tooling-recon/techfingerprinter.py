from httpfetcher import requestit

techheaders = [
    #generic server
    "Server",
    "X-Powered-By",
    "X-Generator",
    "X-AspNet-Version",
    "X-AspNetMvc-Version",
    "X-Runtime",
    "X-Version",
    "X-Backend",
    "X-Server",

    #proxy
    "Via",
    "Forwarded",
    "X-Forwarded-Server",
    "X-Forwarded-Host",
    "X-Forwarded-Proto",
    "X-Real-IP",

    #cache...reverse proxy
    "X-Cache",
    "X-Cache-Hits",
    "X-Served-By",
    "X-Timer",
    "Age",
    "Cache-Status",
    "CDN-Cache-Control",

    #cloudflare
    "CF-Ray",
    "CF-Cache-Status",
    "CF-Worker",
    "CF-Request-ID",
    "Server-Timing",

    #fastly
    "Fastly-FF",
    "Fastly-Debug-Digest",
    "X-Fastly-Request-ID",

    #vercel(next.js)
    "X-Vercel-Id",
    "X-Vercel-Cache",
    "X-Matched-Path",
    "X-Nextjs-Cache",
    "X-Nextjs-Matched-Path",

    #netlify
    "X-Nf-Request-Id",

    #tracing
    "X-Request-Id",
    "X-Correlation-Id",
    "X-Trace-Id",

    # aws/cloudfront/API gateway/s3
    "X-Amz-Cf-Id",
    "X-Amz-Cf-Pop",
    "X-Amz-Request-Id",
    "X-Amz-Id-2",
    "X-Amzn-RequestId",
    "X-Amzn-Trace-Id",
    "X-Amz-Apigw-Id",
    "X-Amz-Bucket-Region",

    #azure
    "X-Azure-Ref",
    "X-Ms-Request-Id",
    "X-Ms-Correlation-Request-Id",
    "X-Ms-Routing-Name",
    "X-Msedge-Ref",

    #google cloud+firebase
    "X-Cloud-Trace-Context",
    "X-GUploader-UploadID",
    "X-Goog-Generation",
    "X-Goog-Metageneration",
    "X-Goog-Stored-Content-Encoding",
    "X-Goog-Stored-Content-Length",
    "X-Goog-Hash",
    "X-Firebase-Cache",

    #kubernetes...
    "X-Envoy-Upstream-Service-Time",
    "X-Kong-Upstream-Latency",
    "X-Kong-Proxy-Latency",
    "X-Upstream-Addr",
    "X-Upstream-Status",
    "X-Upstream-Response-Time",

    #cms
    "X-Drupal-Cache",
    "X-Drupal-Dynamic-Cache",
    "X-WordPress-Cache",
    "X-WP-Total",
    "X-WP-TotalPages",
    "X-Pingback",

    #framework
    "X-Django-Version",
    "X-Laravel-Cache",
    "X-PHP-Version",

    #enterprise apps
    "X-Confluence-Request-Time",
    "X-Seraph-LoginReason",
    "X-AREQUESTID",

    #misc
    "X-UA-Compatible",
    "X-Content-Encoded-By",
    "X-Mod-Pagespeed",
    "X-Turbo-Charged-By",
]

techheaderslower ={item.lower() for item in techheaders}

def techfprinter(domain, bt="chrome"):
    presentheaders= {}
    response = requestit(domain, bt=bt)
    lowerheaders ={key.lower(): value for key, value in response.items()}

    for i in techheaderslower: #go through the techheaders list and checks if one of those headers exists in the response
        if i in lowerheaders:
            presentheaders[i]=lowerheaders[i]

    return presentheaders
        
        
