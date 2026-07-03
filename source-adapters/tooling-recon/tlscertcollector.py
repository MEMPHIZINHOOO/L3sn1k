
# Why is it usefull to collect the TLS certificates
# 1- see for what domain the cert was emitted
# 2- who emitted the cert
# 3- what domains appeared in the SANs (more attack surface maybe)
# 4- if the cert is expired
# 5- if the cert is self-signed
# 6- analyze the chain
# 7- fingerprint the cert
# 8- signing algo
# 9- type and size of the pub key

import socket
import ssl

def tlscertcollect(domain):
#build a tcp connection between me and the server port 443
#extract the certificate and data associated
#build response analysis arround the results
    c = ssl.create_default_context()
    s= socket.create_connection((domain,443), timeout=3)
    ssl_socket = c.wrap_socket(s, server_hostname=domain)
    certificate = ssl_socket.getpeercert()
    tls_version =ssl_socket.version()
    cipher=ssl_socket.cipher()
    ssl_socket.close()
    return {
        "certificate": certificate,
        "tls_version": tls_version,
        "cipher":cipher , 
    }
    
