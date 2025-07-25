# rsync
Rsync is an open-source utility for efficiently transferring and synchronizing files between directories, local 
systems, or remote servers. It’s widely used for backups, mirroring, and data synchronization due to its speed, 
reliability, and flexibility. Rsync is available on Unix-like systems (Linux, macOS) and can be used on Windows 
with tools like Cygwin or WSL.

## What is Rsync?
Rsync (short for "remote sync") is a command-line tool that synchronizes files and directories from one location to 
another while minimizing data transfer by copying only the differences between the source and destination. It uses 
a delta-transfer algorithm to achieve this efficiency, making it ideal for large datasets or slow network connections.

## Key Features of Rsync
1. **Efficient Delta Transfer**: Only transfers changed portions of files, not entire files, reducing bandwidth and time.
2. **Preservation**: Maintains file permissions, timestamps, ownership, and symbolic links (with appropriate flags).
3. **Compression**: Supports compression during transfer to save bandwidth.
4. **Remote and Local Use**: Works for local-to-local, local-to-remote, or remote-to-remote file transfers.
5. **Incremental Backups**: Supports incremental backups with options to preserve older versions of files.
6. **Customizable**: Offers extensive options for filtering, excluding, and controlling the sync process.

### How Rsync Works
Rsync uses a combination of checks to determine which files or parts of files need to be transferred. Here’s a simplified explanation of its process:

1. **File List Comparison**:
   - Rsync builds a list of files and directories in the source and destination.
   - It compares metadata (e.g., file size, modification time) to identify differences.

2. **Delta-Transfer Algorithm**:
   - For files that differ, rsync splits them into fixed-size blocks (e.g., 700 bytes).
   - It calculates checksums (using algorithms like MD5 or SHA-1) for each block in the source and destination.
   - The destination sends its checksums to the source, which compares them to identify changed or new blocks.
   - Only the differing blocks are transferred, not the entire file.

3. **Transfer and Reconstruction**:
   - Rsync sends the changed blocks (or new files) over the network, often compressing the data.
   - The destination reconstructs the updated files using the transferred blocks and existing data.

4. **Metadata Preservation**:
   - Rsync applies the original file’s metadata (permissions, timestamps, etc.) to the destination files, ensuring consistency.

5. **Optional Features**:
   - Rsync can delete files in the destination that no longer exist in the source (with `--delete`).
   - It supports excluding specific files or directories (with `--exclude`).
   - It can operate over SSH for secure transfers or use its own daemon for direct connections.

### Basic Syntax
```bash
rsync [options] source destination
```

- **Source/Destination**: Can be local paths (e.g., `/home/user/dir/`) or remote (e.g., `user@host:/path/`).
- **Common Options**:
  - `-r`: Recursive, syncs directories and their contents.
  - `-a`: Archive mode, preserves permissions, timestamps, symbolic links, etc.
  - `-v`: Verbose, shows detailed output.
  - `-z`: Compresses data during transfer.
  - `--progress`: Shows transfer progress.
  - `--delete`: Deletes files in the destination that are absent in the source.
  - `--exclude`: Excludes specified files or patterns.
  - `-e`: Specifies the remote shell (e.g., `-e ssh` for SSH).

### Examples
1. **Local Sync**:
   ```bash
   rsync -av /home/user/documents/ /backup/documents/
   ```
   Copies the `documents` directory to `/backup/documents/`, preserving permissions and showing progress.

2. **Remote Sync (Push)**:
   ```bash
   rsync -avz -e ssh /home/user/documents/ user@remote:/home/user/backup/
   ```
   Copies `documents` to a remote server over SSH with compression.

3. **Remote Sync (Pull)**:
   ```bash
   rsync -avz -e ssh user@remote:/home/user/backup/ /home/user/documents/
   ```
   Pulls files from the remote server to the local machine.

4. **Exclude Files**:
   ```bash
   rsync -av --exclude '*.log' /home/user/dir/ /backup/dir/
   ```
   Syncs `dir` but excludes `.log` files.

5. **Mirror with Deletion**:
   ```bash
   rsync -av --delete /home/user/dir/ /backup/dir/
   ```
   Mirrors `dir` to `/backup/dir/`, deleting files in the destination that no longer exist in the source.

### How Rsync Optimizes Transfers
- **Rolling Checksum**: Rsync uses a rolling checksum algorithm to compare file blocks efficiently, even if data is shifted (e.g., insertions or deletions in a file).
- **Network Efficiency**: By transferring only changed blocks and optionally compressing data, rsync minimizes bandwidth usage.
- **Idempotency**: Running rsync multiple times produces the same result, making it safe for interrupted transfers.

### Use Cases
- **Backups**: Create incremental backups to save disk space and time.
- **Server Mirroring**: Sync web servers or databases across machines.
- **Data Migration**: Move large datasets between systems with minimal downtime.
- **File Sharing**: Distribute files across distributed teams or devices.

### Caveats
- **Trailing Slash Matters**: `rsync -av /src/dir /dst` copies the `dir` folder into `/dst`, while `rsync -av /src/dir/ /dst` copies the contents of `dir` into `/dst`.
- **Permissions**: Ensure proper user permissions for source and destination, especially for remote syncs.
- **SSH Keys**: For remote transfers, setting up SSH keys can avoid repeated password prompts.
- **Large Datasets**: For very large files or slow networks, consider `--progress` or `--partial` to handle interruptions.

### Advanced Usage
- **Rsync Daemon**: Run rsync as a server for direct connections without SSH.
- **Incremental Backups**: Use `--backup` and `--backup-dir` to store changed files in a separate directory.
- **Bandwidth Limits**: Use `--bwlimit` to cap transfer speed (e.g., `--bwlimit=1000` for 1MB/s).
- **Dry Run**: Use `--dry-run` to simulate the sync without making changes.

For more details, check the rsync man page (`man rsync`) or its official documentation. If you have a specific use case or need help with a command, let me know!

---
# second article
Also a helpful article : <br/>
<a href="https://www.geeksforgeeks.org/linux-unix/rsync-command-in-linux-with-examples/">geeksforgeeks</a>

Syntax of `rsync` command in Linux
```
rsync [options] source [destination]
```

Using `rsync` as a list command: <br/>
If only the source path is specified, the contents of the source are listed in an output format similar to `ls -l`.
```
rsync foo/ 
```

Copy/Sync files and directory locally: <br/>
If neither the source or destination path specifies a remote host, the rsync commands behave as a copy command.
```
rsync -avh foo/ bar/ 
```
If the destination directory is not present , rsync automatically creates one and copies all the data in it.


Rsync using ssh: <br/>
There are two different ways for rsync to contact a remote system:
- Using a remote-shell program as the transport(such as ssh(Secure Shell) or rsh(Remote Shell)).
- Contacting an rsync daemon directly via TCP.

Here we will be discussing rsync over ssh.
```
rsync -avhze ssh /foo user@remote-host:/tmp/
```

Rsync with particular file permissions: <br/>
If we want to sync files to the local or remote host with the permissions of the files being changed. 
The following command must be used.
```
rsync -avhe ssh --chown=USER:GROUP /foo user@remote-host:/tmp/
```
The above command will sync all the files present in directory `/foo` with the files present in directory `/tmp` in 
the remote-host with all the files owned by USER with group GROUP. 

Rsync with `--ignore-existing-files`: <br/>
We can also skip the already existing files on the destination. This can generally be used when we are performing 
backups using the `--link-dest` option, while continuing a backup run that got interrupted.
```
rsync --ignore-existing -avhe /foo user@remote-host:/tmp/
```
So, any files that do not exist on the destination will be copied over.


Show progress during transfer: <br/>
To show the progress while transferring the data from local-host to remote-host, we can use `-–progress` option.
```
rsync -avhe ssh --progress /foo user@remote-host:/tmp/
```


Update the remote only if there is a newer version is on the local filesystem: <br/>
If we want to copy files over the remote-host that have been updated more recently on the local filesystem. It is 
done with `--update` flag. The behavior is now like this:
- Files that do not exist on the remote-host are copied.
- Files that exist on both local and remote but have a newer timestamp on the local-host are copied to 
remote-host. (Conversely, files that have an older timestamp are not copied).

Here, I made some changes in file1 and file2, but the changes in file2 were done recently. So only file2 will get synced.
```
rsync -avhe ssh --progress --update /foo root@remote-host:/tmp/
```


Automatically delete files from local-host after successful transferring:
```
rsync -avhe ssh --remove-source-files /foo user@backup-server:/tmp
```


Delete the files that have been deleted on the local-host:
```
rsync -avhe ssh  /foo --delete user@remote-host:/tmp/
```


Performing a Dry run with `rsync` : <br/>
A Dry run makes `rsync` perform a trial run that doesn't make any changes and displays almost the same output as a 
real run would do. It is generally used with the `-v`, `--verbose` and/or `-i`, `--itemize-changes` options so as to 
see what a `rsync` command would do before one actually runs it.
```
rsync -avhe ssh --dry-run --chown=USER:GROUP /foo user@remote-host:/
```

---
# third article
the third article <a href="https://www.tecmint.com/rsync-local-remote-file-synchronization-commands/">link</a> <br/>




