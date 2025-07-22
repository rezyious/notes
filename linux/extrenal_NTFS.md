# Using NTFS
To use an external NTFS hard drive on Arch Linux, you need to ensure that the `ntfs-3g` package is installed.<br/>
This package allows you to read and write to NTFS file systems. Here's how you can get it working:<br/>

## 1. Install ntfs-3g:
Open a terminal and install `ntfs-3g` with the following command:<br/>
```
sudo pacman -S ntfs-3g
```

## 2. Check if the drive is detected:
Use the `lsblk` command to list all block devices and verify that your external drive is detected.<br/>
```
lsblk
```

## 3. Manually mount the drive:
After verifying the device name (e.g., `/dev/sdX1`), you can mount it manually:<br/>
```
sudo mount -t ntfs-3g /dev/sdX1 /mnt
```

Replace `/dev/sdX1` with the actual device name of your external drive, <br/>
and `/mnt` with the directory where you want to mount it.<br/>

## 4. Check for errors:
If the drive still doesn't mount, you might want to check for filesystem errors on the drive:<br/>
```
sudo ntfsfix /dev/sdX1
```

## 5. Automatic mounting:
If you'd like the drive to mount automatically when connected, you can add an entry to your `/etc/fstab` file:<br/>
```
/dev/sdX1  /mnt  ntfs-3g  defaults  0  0
```

Replace `/dev/sdX1` and `/mnt` with the correct device and mount point. <br/>
This will ensure the drive mounts automatically on boot.<br/>


