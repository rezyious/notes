# AUR

**AUR (Arch User Repository)** and **Yay** in the context of Arch Linux.

## What is AUR?

The **Arch User Repository (AUR)** is a community-driven repository for Arch Linux. It contains packages that are <br/>
not included in the official Arch repositories. AUR allows users to:<br/>
- Access community-maintained software.<br/>
- Install software with user-contributed PKGBUILDs (scripts used to build Arch packages).<br/>
- Use cutting-edge or experimental packages.<br/>

## How to Use AUR?

1. **Manual Method:**
   - Search for a package on the [AUR website](https://aur.archlinux.org/).
   - Download the `PKGBUILD` file for the package.
   - Build and install it using:
     ```bash
     makepkg -si
     ```

2. **Using AUR Helpers:**
   AUR helpers like **Yay** simplify the process by automating the steps above.

## What is Yay?

**Yay (Yet Another Yaourt)** is an **AUR helper** for Arch Linux and its derivatives (like Manjaro). <br/>
It automates the process of:
- Searching for AUR and official repository packages.
- Building and installing AUR packages.
- Keeping AUR packages updated during a system update (`yay -Syu`).

Yay is written in Go and is designed to be fast and user-friendly.


## Installing Yay

You can install Yay by manually building it or using another AUR helper if one is already installed.<br/>

#### Method 1: Manual Installation
1. **Install Prerequisites:**
   Ensure `git` and the `base-devel` group are installed:
   ```
   sudo pacman -S --needed git base-devel
   ```

2. **Clone the Yay Repository:**
   ```
   git clone https://aur.archlinux.org/yay.git
   ```

3. **Build and Install Yay:**
   Navigate into the `yay` directory:
   ```
   cd yay
   makepkg -si
   ```

4. **Verify Installation:**
   Check that Yay is installed:
   ```
   yay --version
   ```

#### Method 2: Use Another AUR Helper
If you already have an AUR helper installed, you can install Yay directly:
```
<aur-helper> -S yay
```
(Replace `<aur-helper>` with your existing AUR helper, such as `paru` or `pamac`.)

---

## Using Yay

Here are common commands with Yay:

1. **Search for a Package:**
   ```
   yay -Ss <package-name>
   ```

2. **Install a Package:**
   ```
   yay -S <package-name>
   ```

3. **Remove a Package:**
   ```
   yay -Rns <package-name>
   ```

4. **Update All Packages (Official + AUR):**
   ```
   yay -Syu
   ```

5. **Clean Unused Dependencies:**
   ```
   yay -Yc
   ```

---

Yay makes managing AUR packages straightforward, saving time and effort compared to manually building them.
