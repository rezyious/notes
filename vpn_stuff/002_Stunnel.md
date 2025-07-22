# Stunnel
First of all make sure firewall is correct and allows ports and protocols required.

## Second Server aka for example EU SERVER config

    sudo apt install stunnel4 openssl -y

  Edit or create this file:

    sudo vi /etc/stunnel/stunnel.conf

Put these in the file :

    cert = /etc/stunnel/stunnel.pem  
    pid = /etc/stunnel/stunnel.pid  
    output = /etc/stunnel/stunnel.log  
      
    [v2ray]  
    accept = 14558  
    connect = 0.0.0.0:39923

In this example the port **14558** is the port that we receive on this server aka for example the EU server
and **39923** is the port that for example we set on **XRAY** for our config.
Basically everything we get on 14558 goes to 39923

Now run these commands:

    sudo openssl genrsa -out privkey.pem 2048  
    sudo openssl req -new -x509 -key privkey.pem -out cacert.pem -days 1095  
    sudo cat privkey.pem cacert.pem >> /etc/stunnel/stunnel.pem  
    sudo chmod 0400 /etc/stunnel/stunnel.pem

  
Now create this service:

    sudo vi /usr/lib/systemd/system/stunnel.service

Enter these in the file :

    [Unit]  
    Description=SSL tunnel for network daemons  
    After=network.target  
    After=syslog.target  
      
    [Install]  
    WantedBy=multi-user.target  
    Alias=stunnel.target  
      
    [Service]  
    Type=forking  
    ExecStart=/usr/bin/stunnel /etc/stunnel/stunnel.conf  
    ExecStop=/usr/bin/pkill stunnel  
      
    # Give up if ping don't get an answer  
    TimeoutSec=600  
      
    Restart=always  
    PrivateTmp=false


Enable and start the service:

    sudo  systemctl  start  stunnel.service  
    sudo  systemctl  enable  stunnel.service

  
  

## First Server aka for example IRAN SERVER config

    sudo apt install stunnel4 openssl -y

  Edit or create this file:

    sudo vi /etc/stunnel/stunnel.conf

Put these in the file :

    pid = /etc/stunnel/stunnel.pid  
    client = yes  
    output = /etc/stunnel/stunnel.log  
      
    [v2ray]  
    accept = 39923  
    connect = <kharej>:14558

Make sure to replace the IP of EU server with **kharej** placeholder

Now create this service:

    sudo vi /usr/lib/systemd/system/stunnel.service

Enter these in the file:

    [Unit]  
    Description=SSL tunnel for network daemons  
    After=network.target  
    After=syslog.target  
      
    [Install]  
    WantedBy=multi-user.target  
    Alias=stunnel.target  
      
    [Service]  
    Type=forking  
    ExecStart=/usr/bin/stunnel /etc/stunnel/stunnel.conf  
    ExecStop=/usr/bin/pkill stunnel  
      
    # Give up if ping don't get an answer  
    TimeoutSec=600  
      
    Restart=always  
    PrivateTmp=false

As you can we do not need to generate private key and cert on the first server aka for example IRAN server.

  
<hr>

**Error client SSL version**
If we get any SSL version error on first server:

    sudo vi /etc/ssl/openssl.cnf

  
Add this to the start of the file:

    openssl_conf = default_conf

  

And these at the end of the file:

    [default_conf]  
    ssl_conf = ssl_sect  
      
    [ssl_sect]  
    system_default = system_default_sect  
      
    [system_default_sect]  
    MinProtocol = TLSv1.2  
    CipherString = DEFAULT@SECLEVEL=1

  
  
Start and enable and if get any error, restart the service:

    sudo  systemctl  start  stunnel.service  
    sudo  systemctl  enable  stunnel.service

  
  
  
  
  
  
  
<hr>
Individual tunnels for each port example.

Edit **stunnel.conf** file accordingly:

First server aka IRAN :

    [v2ray-39923]
    
    accept = 39923
    
    connect = IP-Kharej:14558
    
      
    
    [v2ray-24596]
    
    accept = 24596
    
    connect = IP-Kharej:15558

  
  

Second Server aka EU server :

  

    [v2ray-39923]
    
    accept = 14558
    
    connect = 0.0.0.0:39923
    
      
    
    [v2ray-24596]
    
    accept = 15558
    
    connect = 0.0.0.0:24596
