# Audio
## Linux Audio Explained (ALSA vs PulseAudio vs JACK vs Pipewire Explained)
<a href="https://www.youtube.com/watch?v=HxEXMHcwtlI">Video Link</a>


In the kernel we need a driver to be able to interact with the sound card.<br/>
The whole point of a device driver in the linux kernel is to be able to interact with the sound card. <br/>
In the early days of linux it was done with a driver called `oss` which means `open sound system` .It had some limitations. <br/>
It's  replacement also is what's in use today `alsa` stands for `advanced linux sound architecture` and it's basically what 
supports everything from usb sound cards to pci sound cards to onboard audio on your laptop and desktop.<br/>
alsa provides device drivers necessary to be able to read and write from a sound card <br/>
It doesn't provide much more than that it's just a way to interact with a sound card .<br/>
many applications support working with alsa directly for example, open a video in `vlc` media player and it can <br/>
output audio straight to an alsa device. similarly audacity can directly record from an alsa device.<br/>
In practice this isn't usually what you do because some basic sound cards don't actually support multiplexing so while<br/>
it's great that we can read and write from it, if a program is using alsa it'll just take control of the entire sound card<br/>
so only one program will be able to play or record audio at a time which in general isn't what you want.<br/>
you want to be able to have music playing while you play a video<br/>
or you want to be able to have two separate like programs running that both want to output or record sound <br/>
It would be silly if we had obs running and also had teams except that only one device could use a microphone at a time <br/>
alsa is great it's robust but that limitation of not being able to multiplex sound cards properly has led to replacements <br/>
well not replacements. alsa still remains a driver but we have abstracted a layer above the device driver these days that happens to <br/>
be `pulseaudio` and more recently `pipewire` is replacing pulseaudio<br/>
but we also have `jack` which was for pro audio systems <br/>
now both of these are what we call `sound servers` because the server is going to interact with the sound card and then the programs are the<br/>
clients so each of these client programs can output their sound to <br/>
the `pulseaudio` server and then pulseaudio will take and combine these and the mixing and output it <br/>

-------------------------------------------------
|Program | Pulseaudio | ALSA   |   SoundCard    |
|-------------------------------                |
|client  | server     | driver |                |
-------------------------------------------------

This has many benefits as you might imagine as well as a couple drawbacks ,the benefits are that for no matter what sound card ,even one that<br/>
only supports one end one digital input We can have 10 programs open that are all playing different songs or different videos and `pulseaudio` will <br/>
take all of those , resample them and mix them and then send them to the sound card.<br/>

If you're on a linux computer and you're playing audio chances are it's using pulseaudio , a competitor to pulse audio in the pro audio space <br/>
was `jack`. jack was designed as a virtual essentially patch panel and it's way more flexible than pulseaudio , albeit slightly harder to use.<br/>

Some drawbacks of pulseaudio were that it's hard to have a program say i'll put the two devices at once or connect any input to any output. <br/>

`jack` stands for a `jack audio connection kit` ,obviously recursive acronym. jack will actually do the interacting with sound cards<br/>

Unfortunately pulseaudio and jack often aren't compatible with each other just due to how different the different sound servers are<br/>

`pipewire` has been a great replacement pipewire offers a drop in replacement for pulseaudio.<br/>
any program that used to work with pulseaudio will work with pipewire . The pulseaudio sound server will connect to the pipewire sound server<br/>
and they have the basically the samepulse audio api so no matter which sound server you happen to be using both will work in the same <br/>
way .The benefit of pipewire is how it tends to handle inputs instead of pulseaudio's traditional model, pipewire uses a model that's way <br/>
more reminiscent of jack to the point where it also is a drop in replacement for jack programs. <br/>
So finally for once in the linux ecosystem you can both jack and pulse audio programs simultaneously and they can finally interact with each other properly.<br/>
This is great news because it brings some of the benefits of jack to pulseaudio. <br/>

## Sound system
<a href="https://wiki.archlinux.org/title/Sound_system">Arch Wiki for sound</a><br/><br/>
A default Arch installation already includes the kernel sound system `ALSA`, and lots of utilities for it can be installed from<br/>
the official repositories. If you want additional features you can install one of several sound servers. <br/>

### ALSA
<a href="https://wiki.archlinux.org/title/Advanced_Linux_Sound_Architecture">ALSA Wiki</a><br/><br/>
ALSA is a set of built-in Linux kernel modules. Therefore, manual installation is not necessary.<br/>
`udev` will automatically detect your hardware and select needed drivers at boot time, therefore, your sound <br/>
should already be working. However, your sound may be initially muted. If it is, see #Unmuting the channels. <br/>

#### ALSA firmware
`sof-firmware` is required for some laptop models (mainly since 2019) because they implement their <br/>
drivers with firmware provided by the Sound Open Firmware (SOF) project.<br/>

#### ALSA utilities
Install the `alsa-utils` package.<br/>


#### ALSA and systemd
The `alsa-utils` package comes with systemd unit configuration files `alsa-restore.service` and `alsa-state.service` by default.<br/>
```
systemctl status alsa-state.service
```

#### Pipewire
<a href="https://wiki.archlinux.org/title/PipeWire">Arch Wiki page</a><br/><br/>
PipeWire is a new low-level multimedia framework. It aims to offer capture and playback for both audio and video with<br/>
minimal latency and support for PulseAudio, JACK, ALSA and GStreamer-based applications. <br/>

## Summary for troubleshooting :<br/>

Check available cards with `pactl list cards` <br/>
Set the correct profile using `pactl set-card-profile`<br/>
Check and set the correct output port (headphones, speakers, etc.).<br/>
Ensure the driver im my case (snd_hda_intel) is loaded properly and<br/>
that PipeWire is running.<br/>
Use PipeWire logs (journalctl) for detailed error messages.<br/>
Restart PipeWire services if required.<br/>

Steps for Troubleshooting and Fixing Audio Issues with PipeWire:<br/>

Check the Available Audio Cards: Use `pactl list cards` to list all audio devices. This shows you what <br/>
audio hardware PipeWire is aware of. If your device isn't listed, there could be a driver <br/>
issue or misconfiguration.<br/>
```
aplay - command-line sound recorder and player for ALSA soundcard driver<br/>
pactl - Control a running PulseAudio sound server<br/>
```
<hr/>

```
$ pactl list cards
```
Check the Audio Sinks: Use `pactl list sinks` to list all audio sinks (output devices). The available sinks will <br/>
show up with a state (either playing, suspended, etc.). If your sink is in a suspended state, it may not be <br/>
active for output, and you'll need to set the correct profile or reactivate it.<br/>
```
$ pactl list sinks
```

Ensure Correct Profile is Active: PipeWire can manage multiple profiles for each audio <br/>
device (e.g., output to HDMI, speakers, headphones). Use the command:<br/>
```
$ pactl list cards
```

This will show the available profiles for your audio device. You may need to manually switch to a profile that <br/>
matches your intended output (e.g., output:analog-stereo or output:hdmi-stereo for HDMI audio).<br/>

You can set the profile using:<br/>
```
$ pactl set-card-profile <card_name> <profile_name>
```

For example, to set the profile for your Intel card:<br/>
```
$ pactl set-card-profile alsa_card.pci-0000_00_1f.3 output:analog-stereo+input:analog-stereo
```
Check the Ports: Sometimes, PipeWire may not automatically detect which output (headphones, speakers, HDMI) should <br/>
be used. You can manually set the active port:<br/>
```
$ pactl set-port-latency <port_name> <value>
$ pactl set-port <port_name> <state>
```

Ensure that the correct port for your headphones or speakers is active. For example:<br/>
```
pactl set-port analog-output-headphones available
```

Check the Module and Driver: If PipeWire isn't recognizing the device properly, verify that the driver for the <br/>
audio hardware (e.g., `snd_hda_intel` for Intel audio) is loaded correctly. This can be done with:<br/>
```
$ lsmod | grep snd_hda_intel
```

Ensure PipeWire is Running: Sometimes, PipeWire might not be running or could be <br/>
misconfigured. You can ensure it's running using:<br/>
```
$ systemctl --user status pipewire
```

If not running, start it:<br/>
```
$ systemctl --user start pipewire
```

For WirePlumber (the session manager for PipeWire), check with:<br/>
```
$ systemctl --user status wireplumber
```

Reboot or Restart Audio Service: After making any changes, a quick restart of the audio system can help:<br/>
```
$ systemctl --user restart pipewire
```

Use Logs for Troubleshooting: Always refer to journalctl logs for errors or <br/>
warnings related to PipeWire or audio devices:<br/>
```
$ journalctl --user -xe | grep pipewire
```
This will show if there are any errors related to audio drivers or PipeWire itself.<br/>


## Check PipeWire Dependencies
Ensure all necessary packages are installed:<br/>

```
sudo pacman -S pipewire pipewire-pulse pipewire-alsa pipewire-jack wireplumber
```

Checking PipeWire Service Status to Ensure that PipeWire and its related services are running.<br/>

```
systemctl --user status pipewire
systemctl --user status pipewire-pulse
```

If any of these services are inactive, start and enable them:<br/>

```
systemctl --user start pipewire
systemctl --user start pipewire-pulse
systemctl --user enable pipewire
systemctl --user enable pipewire-pulse
```

## Restart WirePlumber
WirePlumber is the session manager for PipeWire. Restart it:

```
systemctl --user restart wireplumber
```


## Ensure ALSA devices are detected
Check if ALSA sees your devices:<br/>

```
aplay -l
```

If the command is not found , you have to install it<br/>
The aplay command is part of the ALSA utilities, which might not be installed. To fix this:<br/>

### Install ALSA Utilities:

```
sudo pacman -S alsa-utils
```

## Install any missing firmware:

```
sudo pacman -S linux-firmware
```

##  Verify SOF Firmware
Ensure the SOF firmware is installed:<br/>
```
sudo pacman -S sof-firmware alsa-ucm-conf
```

## Check for Audio Devices
Verify if your audio devices are recognized:<br/>
```
pactl list short sinks
```

If no devices are listed, continue to the next step.

## Check for Missing Kernel Modules
Ensure that the appropriate kernel modules for your audio device are loaded:<br/>

```
lsmod | grep snd
```

If no relevant modules are loaded, identify your audio hardware:<br/>

```
lspci | grep -i audio
```



## Revert Configuration Conflicts
Remove any conflicting PulseAudio or ALSA configurations:<br/>
```
rm -rf ~/.config/pulse ~/.config/pipewire ~/.asoundrc
```

Restart PipeWire and WirePlumber:<br/>
```
systemctl --user restart pipewire pipewire-pulse wireplumber
```

Log Out and Back In<br/>


## Check Logs for Errors
Inspect PipeWire logs for errors:<br/>
```
sudo journalctl --user -xe | grep pipewire
```

## Reload ALSA and PipeWire:
```
sudo alsactl restore
systemctl --user restart pipewire pipewire-pulse wireplumber
```

## Check for PipeWire-related issues in the logs:
```
journalctl --user -u pipewire.service -u pipewire-pulse.service -xe
```

After making these changes, reboot your system to ensure the services and kernel modules are fully reloaded:<br/>
```
sudo reboot
```

## Check if the Audio Driver is Loaded
Verify that the kernel modules for your audio hardware are loaded:<br/>
```
lsmod | grep snd
```
If no results are returned. Identify Your Audio Hardware<br/>
Run the following command to detect your audio device:<br/>
```
lspci | grep -i audio
```

If you’re using a USB audio device, also check:<br/>
```
lsusb
```

## Manually Load the Audio Driver
Find the driver associated with your audio device (e.g., `snd_hda_intel`) and try loading it manually:<br/>
```
sudo modprobe snd_hda_intel
```
Replace `snd_hda_intel` with the appropriate driver for your hardware if different.<br/>

### Reinitialize ALSA

Reinitialize ALSA to detect devices:<br/>
```
sudo alsactl init
```

Check Logs for Errors<br/>
Inspect system logs to diagnose issues:<br/>
```
dmesg | grep snd
```
This will show if there are any errors related to the sound driver.<br/>

After making the changes, reboot the system:<br/>
```
sudo reboot
```

If the issue persists,use these to diagnose:<br/>
```
lsmod | grep snd
lspci | grep -i audio
dmesg | grep snd
```

## Force Reload the Audio Modules
Sometimes, reloading the drivers can resolve detection issues:<br/>
```
sudo modprobe -r snd_hda_intel snd_soc_dmic
sudo modprobe snd_hda_intel snd_soc_dmic
```
Check the Kernel Logs<br/>
Inspect the logs for any errors related to audio:<br/>
```
dmesg | grep snd
```
Look for errors or warnings that might explain why your hardware isn't initializing correctly.<br/>


## Install and Use alsa-tools
Use ALSA utilities to manually detect and initialize the sound card:<br/>
```
sudo pacman -S alsa-tools alsa-utils
sudo alsactl init
```

Verify PipeWire Setup<br/>
Ensure that PipeWire is correctly detecting your hardware:<br/>
```
pactl list cards
pactl list sinks
```

If no cards or sinks are listed, it could indicate an issue with PipeWire’s configuration.<br/>


## Disable SND_SOC_DMIC (Optional)
On some systems, the `snd_soc_dmic` module can conflict with `snd_hda_intel`. Try blacklisting it:<br/>
Edit the configuration file:<br/>
```
sudo nano /etc/modprobe.d/blacklist-dmic.conf
```

Add the following line:<br/>
```
blacklist snd_soc_dmic
```

Rebuild the initramfs:<br/>
```
sudo mkinitcpio -P
```

Reboot your system.<br/>


## Verify ALSA Devices
Check if ALSA detects your audio devices:<br/>
```
cat /proc/asound/cards
```
if it lists your sound card(s), the problem is likely between ALSA and PipeWire. If it <br/>
says "No soundcards found," continue with kernel module troubleshooting.<br/>


## Check Logs for Errors
Look for errors or warnings in the system logs:<br/>
```
journalctl --user -xe | grep pipewire
journalctl -b | grep snd
```

## Check SOF Configuration
Ensure the system is correctly configured to use the SOF driver. Create or edit the following file:<br/>
```
sudo nano /etc/modprobe.d/alsa.conf
```
Add this line to explicitly use the SOF driver:<br/>
```
options snd_intel_dspcfg dsp_driver=1
```

Rebuild Initramfs<br/>
After modifying driver options, rebuild the initramfs to apply the changes:<br/>
```
sudo mkinitcpio -P
```

Restart Services<br/>

Restart the audio services:<br/>
```
systemctl --user restart pipewire pipewire-pulse wireplumber
```

Reboot your system to ensure all changes take effect:<br/>
```
sudo reboot
```

Test ALSA and PipeWire After rebooting:<br/>
Check ALSA for devices:<br/>
```
cat /proc/asound/cards
```

Check PipeWire for devices:<br/>
```
pactl list cards
pactl list sinks
```

## Additional Debugging
If the issue persists, check for errors in the SOF driver:<br/>
```
dmesg | grep sof
```
