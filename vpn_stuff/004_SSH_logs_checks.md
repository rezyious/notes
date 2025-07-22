# Checking ssh logs:

Check the file
```
sudo vi /var/log/auth.log
```

or
```
sudo cat /var/log/auth.log |  nl | less
```
## Password
If you use password , this list the attackers:
```
grep "Failed password for" /var/log/auth.log | grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | sort | uniq -c
grep "Failed password for" /var/log/auth.log | grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | sort | uniq -c | nl | less
```


To check all the IPS:
```
sudo cat /var/log/auth.log |  grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | wc -l
sudo cat /var/log/auth.log |  grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | uniq | wc -l
```

"uniq -c" counts the number of occurences:
```
sudo cat /var/log/auth.log |  grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | uniq -c | wc -l
sudo cat /var/log/auth.log |  grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | uniq -c | less
sudo cat /var/log/auth.log |  grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | uniq -c | nl | less
```
## SSH Keys
Since I use ssh keys and password auth is disables:
```
sudo cat /var/log/auth.log | grep preauth |  grep -Po "[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" | uniq | wc -l
```

check an IP for example my IP:
```
sudo cat /var/log/auth.log | grep 50.212.24.195
```

Check for successfull ones:
```
sudo cat /var/log/auth.log | grep Accepted
```
