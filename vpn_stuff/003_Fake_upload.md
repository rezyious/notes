# Fake Upload
Install nload <br/>

```
sudo apt install nload
```

Generate ssh keys and add to second server
make sure to use ssh-agent on the first server

run any of the scripts on the first server:
```
$ ./fake-1GiB.sh
```
Or
```
$ bash fake-1GiB.sh
```


## fake-1GiB.sh file :
Create a 1GB file <br/>
```
dd if=/dev/zero count=1024 bs=1048576 | ssh root@<second_server> 'cat > /dev/null'
```

## Create a 10GB file
```
dd if=/dev/zero count=1024 bs=10485760 | ssh root@<second_server> 'cat > /dev/null'
```

## Create a 100GB file
```
dd if=/dev/zero count=1024 bs=104857600 | ssh root@<second_server> 'cat > /dev/null'
```

### Use crontabe to automate:
```
$ crontab -e
```

checkout this site: <br/>
https://crontab.guru/


Edit for example like this <br/>
```
0 0 * * * /bin/bash /root/fake-1GiB.sh
```

 

