# systemd

## How to identify the init system:
Look for the **/run/systemd/system/** directory. If this exists, then your init system is systemd.<br/>

or<br/>
```
stat /sbin/init
```

or<br/>
```
sudo stat /proc/1/exe
```

or<br/>
```
cat /proc/1/comm
```

or<br/>
```
ps -p 1
```

Also check these commands<br/>
```
ps -ef
```

and<br/>
```
pstree -p
```

To view a single process and its parents and children and threads.<br/>
For example I find the PID of NetworkManager and use `pstree`:`<br/>
```
pstree -sp 544
```
Obviously the PID is diffrent every time.<br/>


## Listing Services and Their States with systemctl
list of all loaded units<br/>
```
systemctl
```

listing all units, active and inactive:<br/>
```
systemctl --all
```


How many total unit files<br/>
```
systemctl list-unit-files
```

and use --user for user services:<br/>
```
systemctl --user list-unit-files
```
for example to lookup *pipewire* if you using it.<br/>



just the service files<br/>
```
systemctl list-unit-files --type=service
```

List only enabled services:<br/>
```
systemctl list-unit-files --type=service --state=enabled
```

or disabled ones<br/>
```
systemctl list-unit-files --type=service --state=disabled
```

Service unit files are in <br/>
`/usr/lib/systemd/system/` or `/lib/systemd/system/`<br/>
according to where your Linux distribution puts them. These are plain-text files you can read.<br/>


When a service is enabled, systemd creates a symlink in `/etc/systemd/system/` rom the unit file in `/lib/systemd/system/`<br/>


## Querying the Status of Selected Services and Stopping and Starting
Example<br/>
```
systemctl status cups.service
```

Query multiple services with a space-delimited list:<br/>
```
systemctl status mariadb.service bluetooth.service lm-sensors.service
```

The following examples use the SSH service to demonstrate service management.<br/>
```
sudo systemctl start sshd.service
sudo systemctl stop sshd.service
sudo systemctl restart sshd.service
```

Reload the service’s configuration. For example, you made a change to sshd_config and want to load the new <br/>
configuration without restarting the service:<br/>
```
sudo systemctl reload sshd.service
```

All of these commands also work with multiple services, space-delimited, for example:<br/>
```
sudo systemctl start sshd.service mariadb.service firewalld.service
```

## Enabling and Disabling Services
Example:
```
sudo systemctl enable sshd.service
```

This does not start the service. You can start the service with systemctl start, or enable and start the service <br/>
in one command with the `--now` option:<br/>
```
sudo systemctl enable --now sshd.service
```

The same thing goes for disable, so you have to stop it or just use this command<br/>
```
sudo systemctl disable --now sshd.service
```

This command reenables the mariadb service, which disables and then enables it. If you have created the symlinks <br/>
manually for a service, this is useful for quickly reset‐ ting them to the defaults:<br/>
```
sudo systemctl reenable mariadb.service
```

The following command disables the bluetooth service completely by masking it, so that it cannot be started at all:<br/>
```
sudo systemctl mask bluetooth.service
```

Unmasking the bluetooth service does not enable it, so it must be started manually:<br/>
```
sudo systemctl unmask bluetooth.service
sudo systemctl start bluetooth.service
```

## Stopping Troublesome Processes
`systemctl kill` is preferable because it stops all processes that belong to a service and leaves no orphan processes,
nor any processes that might restart the service and continue to make trouble. First, try it with no options other 
than the service name, then check the status:
```
sudo systemctl kill mariadb
```

If this does not work, then try the nuclear option:
```
sudo systemctl kill -9 mariadb
```
The legacy kill command does not recognize service or command names, but rather requires the PID of the offending process:
```
sudo kill 1234
```
If this does not stop it, use the nuclear option:
```
sudo kill -9 1234
```

## --user flag
 The difference between using `systemctl` with `--user` and without it relates to the scope in which the service
 operates: **user-level** or **system-level**. Here’s a breakdown of why this distinction exists
 and how it applies to services like `pipewire`:

### 1. **System-Level Services (default, without `--user`)**
- These are managed by the system instance of `systemd` and are typically started during boot.
- They operate at the system level, affecting all users on the system.
- Services here are usually critical system daemons like `networking`, `ssh`, or `httpd`.
- To manage these services, administrative privileges (root) are required.

Example:<br/>
```
sudo systemctl start sshd.service
```

### 2. **User-Level Services (with `--user`)**
- These are managed by the user instance of `systemd` and are specific to individual users.
- Services here are run only for a particular user and usually start when the user logs in.
- No administrative privileges are required to manage these services for the current user.
- Examples include desktop-related or session-specific services like `pipewire`, `dbus`, and 
  other user-facing utilities.

Example:<br/>
```
systemctl --user start pipewire.service
```

### Why `pipewire` uses `--user`
`PipeWire` is a multimedia framework commonly used for audio and video processing. It is designed to operate in 
  the **user session** because:
- It often needs access to the user's graphical session, sound devices, and preferences, which are 
  user-specific resources.
- Running it at the system level would complicate access control and isolate it from user-level sessions,
  leading to issues with multi-user environments or sandboxed applications.

### Services That Don't Use `--user`
Services like `nginx`, `mysql`, or `sshd` operate at the **system level** because:
- They provide system-wide functionality.
- They often need privileges to bind to system-level resources like ports or hardware 
  devices that are shared across users.

### How to Know When to Use `--user`
- Use `--user` if the service is primarily tied to a specific user session (e.g., PipeWire, graphical services).
- Use system-level (no `--user`) for services meant to operate across all users or for essential system functions.

You can also check the service definition to confirm its scope:
- For system-level services: `/lib/systemd/system/` or `/etc/systemd/system/`.
- For user-level services: `~/.config/systemd/user/` or `/usr/lib/systemd/user/`.

To list all services for the current user:
systemctl --user list-units

To list system-wide services:
systemctl list-units

## systemctl edit

`systemctl edit` is a command used to customize and modify systemd service files, allowing you to override
or append settings without directly modifying the original service files. This approach is recommended 
because it ensures your customizations are preserved even if the original service file is updated by a package manager.

### **How `systemctl edit` Works**
When you use `systemctl edit`, systemd creates or opens an override file for the selected service. This override 
file is separate from the original service file and allows you to specify changes or additions to the service 
configuration.

### **Why Use `systemctl edit`?**
1. **Preserve Upgrades:** Changes are made in a separate override file, so the original service file remains 
   untouched and can be safely updated by the package manager.
2. **Ease of Management:** Only your customizations are stored in the override file, 
   making it easier to track what you've changed.
3. **Rollback Capability:** You can easily remove your changes by deleting the override file.

### **Usage Syntax**
systemctl edit <service_name>

- Example:
```
  sudo systemctl edit nginx.service
```        

This command opens a text editor (usually your default editor like `nano` or `vim`) where you can 
specify your overrides.

### **How to Use It**

#### 1. **Add or Override Settings**
When you run the command, you’re editing the **override file** (not the main service file). You only need to specify 
the settings you want to change or add.

For example, to add an environment variable to a service:
[Service]
Environment="MY_VAR=my_value"

This will append the `Environment` variable to the service's `[Service]` section.

#### 2. **Viewing Full Configuration**
To see the complete configuration (including overrides), use:
systemctl cat <service_name>
This will show the original service file followed by any overrides.

#### 3. **Remove Overrides**
If you want to remove your customizations, delete the override file:
sudo systemctl edit --full <service_name>

Delete the content or use:
sudo rm /etc/systemd/system/<service_name>.d/override.conf

---

### **Common Use Cases**
1. **Modify Service Behavior**
   Example: Changing the restart policy of a service:
   [Service]
   Restart=always

2. **Adjust Resource Limits**
   Example: Limiting CPU usage for a service:
   [Service]
   CPUQuota=50%

3. **Add Dependencies**
   Example: Adding a dependency on another service:
   [Unit]
   Requires=network-online.target
   After=network-online.target

### **Types of Edits**
1. **Default Edit (Override Only):**
   This creates or modifies an override file in `/etc/systemd/system/<service_name>.d/override.conf`.

2. **Full Edit:**
   If you need to edit the complete service file, use:
   sudo systemctl edit --full <service_name>

   This creates a full copy of the service file in `/etc/systemd/system/` that can be fully customized. 
   Be cautious, as this replaces the entire service configuration.


### **After Editing**
Once you've made changes:
1. Reload the systemd daemon to apply the changes:
   sudo systemctl daemon-reload

2. Restart the service to apply the new configuration:
   sudo systemctl restart <service_name>

### **Key Points to Remember**
- Always prefer `systemctl edit` over directly modifying service files 
  in `/lib/systemd/system/` or `/usr/lib/systemd/system/`.
- Use `systemctl daemon-reload` after making changes to inform systemd of the updates.
- Test changes carefully, especially when using `--full`, as it completely replaces the service definition.

