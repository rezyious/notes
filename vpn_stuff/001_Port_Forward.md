# Port Forward
Make sure iptables is installed:

    sudo apt update
    sudo apt upgrade  
    sudo apt install iptables

create this file if it does not exist:

    sudo vi /etc/rc.local

Put these in and make sure the IPs are correct:

    #!/bin/bash  
    sysctl net.ipv4.ip_forward=1  
    iptables -t nat -A PREROUTING -p tcp --dport 22 -j DNAT --to-destination <SERVER_1_IP>  
    iptables -t nat -A PREROUTING -j DNAT --to-destination <SERVER_2_IP>  
    iptables -t nat -A POSTROUTING -j MASQUERADE  
    exit 0

Make **rc.local** executable :

    sudo chmod +x /etc/rc.local


instead of this line in the bash script:

    sysctl net.ipv4.ip_forward=1

we can edit this file **/etc/sysctl.conf** permanently :

    sudo vi /etc/sysctl.conf

change this line. Add or uncomment or comment

    net.ipv4.ip_forward=1

This is an example for specific ports to be forwarded:

    #! /bin/bash
    
    #This enables IP Forwarding
    sysctl net.ipv4.ip_forward=1
    
    
    # This is for accessing server via ssh
    # if you use another ssh port, specify it here
    # replace <IP_Server_1> with your for example iran server
    iptables -t nat -A PREROUTING -p tcp --dport 22 -j DNAT --to-destination <IP_Server_1>
    
    # In this example we forward port 443 to the server specified , change the IP accordingly
    # Note that both tcp and udp traffic are forwarded
    iptables -t nat -A PREROUTING -p tcp --dport 443 -j DNAT --to-destination 65.109.186.14:443
    iptables -t nat -A PREROUTING -p udp --dport 443 -j DNAT --to-destination 65.109.186.14:443
    
    # And another port
    iptables -t nat -A PREROUTING -p tcp --dport 34120 -j DNAT --to-destination 65.109.186.14:34120
    iptables -t nat -A PREROUTING -p udp --dport 34120 -j DNAT --to-destination 65.109.186.14:34120
    
    
    iptables -t nat -A POSTROUTING -j MASQUERADE
    exit 0


**Port Forwarding and proxy jump** :

Port forward:

    $ ssh -f -N -D  8080 root@vpsip  

   
Proxy jump:

    $ ssh -J reza@IRAN_VPS_IP root@EU_VPS_IP -f -N -D 8080
