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
