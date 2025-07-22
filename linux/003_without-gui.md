# Start withou GUI

## 1. **Set the Default Target to Multi-User**
The systemd `multi-user.target` is analogous to the traditional "runlevel 3" (non-graphical multi-user mode). Follow these steps:

1. **Check the Current Target:**
   ```
   systemctl get-default
   ```
   This will typically return `graphical.target` if your system boots into a graphical interface by default.

2. **Change the Default Target:**
   To set the default to `multi-user.target` (non-graphical mode):
   ```
   sudo systemctl set-default multi-user.target
   ```
   This ensures the system boots into a terminal (console) by default.


## 2. **Manually Start the Graphical Interface**
When you need to start the graphical interface:

1. **Start the Display Manager (e.g., GDM, SDDM, LightDM):**
   ```
   sudo systemctl start <display-manager>.service
   ```
   Replace `<display-manager>` with the name of your display manager, such as `gdm`, `sddm`, or `lightdm`. For example:
   ```
   sudo systemctl start gdm.service
   ```

2. **Alternatively, Start X Manually:**
   If you don’t use a display manager and prefer starting X manually:
   ```
   startx
   ```
   Ensure that you have a proper `~/.xinitrc` file configured to launch your preferred desktop environment or window manager (e.g., `exec i3`, `exec startkde`, or `exec gnome-session`).

---

## 3. **Switch to Graphical Mode on Demand**
To temporarily switch to the graphical target without changing the default:
```
sudo systemctl isolate graphical.target
```

---

## 4. **Revert to Graphical Target as Default**
If you later decide to always boot into the graphical interface, revert the default target:
```
sudo systemctl set-default graphical.target
```


## 5. **Optional: Boot-Time Override**
If you occasionally want to boot into graphical mode while the default is set to non-graphical mode:

1. **During Boot:**
   At the GRUB menu, edit the boot entry (press `e` on the highlighted entry) and add:
   ```
   systemd.unit=graphical.target
   ```
   to the kernel parameters.

2. **Boot into Non-Graphical Mode Temporarily:**
   Similarly, if `graphical.target` is the default, you can add:
   ```
   systemd.unit=multi-user.target
   ```

This approach provides flexibility to choose between graphical and non-graphical environments.


## Without a DE
Lets say we install a base arch and we dont plan on installing a Desktop Environment and want to use a window manager
like `sway`. In this case we actually dont need a display manager like `gdm` or `lightdm`. We can start sway manually.

After logging in to the terminal (TTY), you can start Sway by simply typing:
```
sway
```


