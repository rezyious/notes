# GPG
This video introduction is useful:
<a href="https://youtu.be/eLKOIjNFwVs?si=PLbj2rqOqCVmM92R">link</a>
<br/>

## Generating GPG keys
from manpage:
```
 --full-gen-key
Generate a new key pair with dialogs for all options.  This is an extended version of --generate-key.
There is also a feature which allows you to create keys in batch mode. See the manual section 
‘‘Unattended key generation'' on how to use this.
```

So to create the key:
```
gpg --full-gen-key
```
Best to choose `ECC` with **sign and encrypt** which is the default. Select the elliptic curve.Next you can select and
expiration date, the default is `0` which means the key doesnt expire. Next answer ther questions to construct your
user identity. Finally type `O` for Okay and select a passphrase.

it creates a key pair (public and private keys), which are stored in your local GPG keyring.
By default, GPG stores keys in the following directory:
```
~/.gnupg/
```
Key files inside that folder:
- `pubring.kbx` – stores public keys
- `private-keys-v1.d/` – stores private keys
- `trustdb.gpg` – stores trust information

View public keys:
```
gpg --list-keys
```

View private keys:
```
gpg --list-secret-keys
```

Each key listing will show a key ID or fingerprint like this:
```
pub   rsa3072 2025-07-23 [SC]
      0123ABCD4567EFGH89IJ0123ABCD4567EFGH89IJ
uid           [ultimate] John Doe <john@example.com>
```

Export your keys;
Export public key:
```
gpg --armor --export <KEY_ID>
```

Export private key:
```
gpg --armor --export-secret-keys <KEY_ID>
```
Replace `<KEY_ID>` with the actual key ID or email, e.g., `john@example.com`.

Delete your keys: <br/>
It always prompts you to first delete the private keys and then the public key because if you get rid of private key
then the public key is useless <br/>
Delete public key:
```
gpg --delete-key <KEY_ID>
```

Delete private key:
```
gpg --delete-secret-key <KEY_ID>
```
⚠️ If you only delete the public key, the private key will still remain. To fully delete a key pair, delete both.


If you want to completely reset all GPG keys, you can remove the .gnupg directory (⚠️ be careful!):
```
rm -r ~/.gnupg
```
Only do this if you're sure you want to wipe all keys and trust data.



## Encrypting and decrypting:
For these examples I created keys with no name and email and comment and use the User id.<br/>

### Encrypting
```
gpg --encrypt --recipient 7B9DB78EC40107951FB5460B42A7530FD505011C arch-nord-dark.png
```
This creates an encrypted file called: `arch-nord-dark.png.gpg`

You can also use ASCII output (text format) with `--armor`:
```
gpg --armor --encrypt --recipient <KEY_ID> arch-nord-dark.png
```
This creates `arch-nord-dark.png.asc`.


### Decrypting
To decrypt the file (e.g., arch-nord-dark.png.gpg):
```
gpg --output decrypted.png --decrypt arch-nord-dark.png.gpg
```

Compare original and decrypted file:
```
diff arch-nord-dark.png restored.png
```
If there's no output, they are identical.<br/>

**Note**: Here we used our public key to encrypt the file and later on decrypted with our private key. Bu in reality
if we want to send an encrypted message or file to someone, we use their public key and encrypt the message and send
it to them and they use their private key to decrypt it. <br/>

Another example:
```
 echo "this message wil be encrypted" \
| gpg --encrypt --armor --recipient 7B9DB78EC40107951FB5460B42A7530FD505011C \
| gpg --decrypt --armor
```
So here we first encrypt the message and immediately decrypt it for testing purposes. We could also save it to a file
and decrypt it separately and not in one go.

`--armor` explanation from manpages:
```
--armor
-a     Create ASCII armored output.  The default is to create the binary OpenPGP format.
```

Example: 
```
echo "this message wil be encrypted" \
| gpg --encrypt --armor --recipient 7B9DB78EC40107951FB5460B42A7530FD505011C > encrypted_message.txt
```

To decrypt and view:
```
cat encrypted_message.txt | gpg --decrypt
```
if you want you can pipe the output to a file ` > decrypted_message.txt ` . <br/>

## Signing/Verifying Messages and Files
Signing uses your private key, and others verify the signature using your public key. Here’s a clear breakdown of the 
process of signing and verifying a file with GPG: <br/>

WHY SIGN?
- Authentication – Proves you created the file.
- Integrity – Verifies the file wasn’t modified.
- Non-repudiation – You can’t deny creating it later.

Some explanation from manpage:
```
 --clearsign
Make a cleartext signature.The content in a cleartext signature is readable without any special software. OpenPGP 
software is only needed to verify the signature.cleartext signatures may modify end-of-line whitespace for platform
independence and are not intended to be reversible.The signing key is chosen by default or can be set explicitly using
the --local-user and --default-key options.
```

```
 --local-user name
-u     Use name as the key to sign with. Note that this option overrides --default-key.
```

Example:
create the signed file:
```
echo "this is a test" \
| gpg --clearsign -u 7B9DB78EC40107951FB5460B42A7530FD505011C > signed_message.txt
```

we can verify it like this:
```
gpg --verify signed_message.txt
```
If everything is ok and the file is not tampered with, we get a message like this:
```
gpg: Signature made Wed 23 Jul 2025 10:22:58 AM +0330
gpg:                using EDDSA key 7B9DB78EC40107951FB5460B42A7530FD505011C
gpg: Good signature from " (my key for learning purposes)" [ultimate]
```
If we mess with the message and change it we get a `BAD signature` , also if we mess with the signature , that is also
an error.

Now a file example:
```
 gpg --sign -u 7B9DB78EC40107951FB5460B42A7530FD505011C arch.png
```

To verify , same as before:
```
 gpg --verify arch.png.gpg
```

if you want the file and signature be separate:
```
gpg --detach-sign -u 7B9DB78EC40107951FB5460B42A7530FD505011C arch.png
```
This wil create a separate file with a `.sig` extension and in our example `arch.png.sig`.

To verify:
```
gpg --verify arch.png.sig arch.png
```
And it passes. <br/>
Now for testing, mess with file in GIMP and you see that `verify` fails.








