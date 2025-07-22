# Terminal Proxy

**Configure Terminal Proxy**:

You can use proxychains to route your terminal traffic through the VPN. First, install proxychains:

    sudo apt-get install proxychains

  


Configure proxychains to Use the VPN:

Edit the proxychains configuration file:

    sudo nano /etc/proxychains.conf

  

Add the following line at the end of the file:

    socks5 127.0.0.1  1080 # Use the appropriate port

Save and exit the file.

  

**Use proxychains with Commands**:

Now, you can use proxychains with terminal commands to route traffic through the VPN. For example:

    proxychains apt update  
    proxychains apt install <package-name>


Replace <package-name> with the actual package you want to install.

  

Verify Your IP:

    curl icanhazip.com
    curl ifconfig.me
    curl ipecho.net/plain

  
  
To use proxychains:

    proxychains curl icanhazip.com


You can use an alias to avoid typing proxychains every time. You can define an alias in your shell configuration file (e.g., .bashrc or .zshrc). 
