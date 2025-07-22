# OBS - Wayland
OBS Studio on wayland requires specific configuration because Wayland uses a different mechanism for screen capture <br/>
than X11. Here are the steps to troubleshoot and fix the issue:<br/>

---

### 1. **Install Required Packages**
Ensure you have the necessary libraries and tools for OBS and Wayland:

```
sudo pacman -S obs-studio qt5-wayland wlroots
```

---

### 2. **Enable PipeWire**
Wayland screen capture in OBS typically uses **PipeWire**, which needs to be set up correctly.

1. **Install PipeWire and dependencies**:
   ```
   sudo pacman -S pipewire pipewire-pulse xdg-desktop-portal xdg-desktop-portal-wlr
   ```

2. **Start and Enable PipeWire Services**:
   Ensure the `pipewire` and `xdg-desktop-portal` services are running:
   ```
   systemctl --user enable --now pipewire pipewire-pulse xdg-desktop-portal xdg-desktop-portal-wlr
   ```

---

### 3. **Configure OBS for Wayland**
When you launch OBS, ensure it’s using the Wayland backend:
- Run OBS with the Wayland flag:
  ```
  obs --enable-wayland
  ```
- Alternatively, create a `.desktop` launcher file for OBS to include this flag.

---

### 4. **Allow Screen Capture Permissions**
Wayland restricts direct screen capture for security. You need `xdg-desktop-portal-wlr` to allow OBS to capture windows or the entire screen:
- **Set environment variables**: Add the following to your shell configuration file (`~/.bashrc`, `~/.zshrc`, or `~/.profile`):
  ```
  export XDG_CURRENT_DESKTOP=sway
  export XDG_SESSION_TYPE=wayland
  ```
  Then, restart your session.

- **Configure Sway**: Ensure your `sway` configuration has the following settings to allow screen capture:
  ```
  exec --no-startup-id /usr/libexec/xdg-desktop-portal-wlr
  ```

---

### 5. **Test Screen Capture**
1. Open OBS Studio.
2. Add a new source:
   - **Screen Capture (PipeWire)** for capturing the screen.
   - **Window Capture (PipeWire)** for capturing specific windows.
3. Select the desired screen or window when prompted.

---

### 6. **Check Logs for Errors**
If it still doesn't work, check OBS logs to identify issues:
- Start OBS from the terminal:
  ```
  obs --enable-wayland
  ```
- Look for any error messages related to PipeWire or screen capture.

---

### 7. **Debugging Tips**
- **Reinstall Dependencies**: If something’s missing or broken, reinstall the required packages:
  ```
  sudo pacman -Syu obs-studio pipewire xdg-desktop-portal xdg-desktop-portal-wlr
  ```
- **Verify Wayland Session**: Ensure you are logged into a Wayland session (`echo $XDG_SESSION_TYPE` should return `wayland`).

