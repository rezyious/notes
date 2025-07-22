# Connecting Android phone to Arch Linux 


## File Transfer

1. **Install Required Tools**:
   - Use `mtp` (Media Transfer Protocol) for managing file transfers.
   - Install `gvfs-mtp` and `mtpfs`:
     ```
     sudo pacman -S gvfs-mtp mtpfs
     ```

2. **Connect Your Phone**:
   - Plug your phone into your computer using a USB cable.
   - On your phone, select "File Transfer (MTP)" in the USB options.

3. **Access Files**:
   - Open your file manager (e.g., `thunar`, `nautilus`) or mount manually:
     ```
     mtp-detect
     mtpfs ~/mnt/android
     ```


