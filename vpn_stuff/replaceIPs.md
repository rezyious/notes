# Create a set of new X-ray configs from IPs
need an `original.txt` file like this of for example a vless config:
```
vless://<UUID>@<DOMAIN>:443?encryption=none&security=tls&sni=<SNI>&alpn=h2%2Chttp%2F1.1&fp=chrome&type=ws&host=<DOMAIN>&path=%2Fapi#vless-ws-tls-443-reza-pc
```

need an `ips.txt` like this:
```
162.159.10.89
170.114.46.73
188.114.98.249
104.24.74.81
```

You can get IPs from CFScanner or on web <br/>
finally the results will be saved in a `results.txt` which you can copy paste to v2rayng <br/>


## Python Script
```
#Python code

import re

# Function to replace the IP address in the URL
def replace_ip(url, new_ip):
    # Regex pattern to find an IPv4 estdressed in the URL
    pattern = r'(?<=@)[^:]+'
    # Replace the found IP address with new_ip
    new_url = re.sub(pattern, new_ip, url)
    return new_url

# Load the original URL
with open('original.txt', 'r') as file:
    url = file.read().strip()

# Load the new IPs
with open('ips.txt', 'r') as file:
    ips = file.read().splitlines()

# Open the output file
with open('results.txt', 'w') as outfile:
    # Process each new IP and write the result to the output file
    for ip in ips:
        new_url = replace_ip(url, ip)
        outfile.write(new_url + '\n')

print("All IPs have been processed and results are saved in 'results.txt'.")
```
