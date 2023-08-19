# passport-desktop

Windows Hello for client Applications in Node.js on Windows Platforms.
This is a replacement for the [node-ms-passport](https://github.com/MarkusJx/node-ms-passport) module.

This module's implementation was heavily inspired by the
[desktop module of bitwarden](https://github.com/bitwarden/clients/tree/master/apps/desktop/desktop_native).

## Installation

```bash
npm install passport-desktop
```

Pre-built binaries are available for windows 32/64 bit platforms.
On other platforms, the module is obviously not available,
importing it will **not** throw an error. Instead, all methods will
throw an error when called, except for `Passport.available()` which
will return `false`.

## Usage

### Check if Windows Hello is available

```ts
import { Passport } from 'passport-desktop';

if (!Passport.available()) {
    throw new Error('Windows Hello is not available');
}
```

### Check if an Passport account with a given id exists

```ts
import { Passport } from 'passport-desktop';

await Passport.accountWithIdExists('my-account-id'); // false, probably
```

### Create a new Passport account and sign a challenge

```ts
import {
    Passport,
    PublicKeyEncoding,
    KeyCreationOption,
} from 'passport-desktop';
import { randomBytes, createPublicKey, createVerify } from 'node:crypto';

const passport = new Passport('my-account-id');
if (!passport.accountExists) {
    await passport.createAccount(KeyCreationOption.FailIfExists);
}

const challenge = randomBytes(32);
const signature = await passport.sign(challenge);

// Verify the signature with the public key
const keyBuffer = await passport.getPublicKey(
    PublicKeyEncoding.Pkcs1RsaPublicKey
);
const key = createPublicKey({
    key: keyBuffer,
    format: 'der',
    type: 'pkcs1',
});

// Create a verifier and verify the challenge
const verify = createVerify('SHA256');
verify.write(challenge);
verify.end();

verify.verify(key, signature); // true

// Delete the account
await passport.deleteAccount();
```

### Verify a challenge signed by a client

A challenge signed by a client can be verified by using the public key of the client.
The node-crypto module may be used to verify the signature.
The public key can be obtained by the client by calling `Passport.getPublicKey()` and
passing the `PublicKeyEncoding.Pkcs1RsaPublicKey` encoding option to that method.

```ts
import { randomBytes, createPublicKey, createVerify } from 'node:crypto';

const challenge = randomBytes(32);

// Send the challenge to the client and obtain the signature

const keyBuffer: Buffer = ...; // Obtain the public key from the client
const signature: Buffer = ...; // Obtain the signature from the client

const key = createPublicKey({
    key: keyBuffer,
    format: 'der',
    type: 'pkcs1'
});

const verify = createVerify('SHA256');
verify.write(challenge);
verify.end();

verify.verify(key, signature);
```
