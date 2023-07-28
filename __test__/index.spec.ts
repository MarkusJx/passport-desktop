import test from 'ava';

import {
    KeyCreationOption,
    Passport,
    PublicKeyEncoding,
    VerificationResult,
} from '../.';
import { createPublicKey, createVerify, randomBytes } from 'crypto';
import isCi from 'is-ci';

const MODULE_NOT_FOUND = {
    code: 'MODULE_NOT_FOUND',
    message: /^Cannot find module '.+'$/m,
};

const windowsTest = process.platform === 'win32' ? test : test.skip;
const windowsLocalTest =
    process.platform === 'win32' && !isCi ? test : test.skip;
const unixTest = process.platform !== 'win32' ? test : test.skip;

windowsTest('available', (t) => {
    t.notThrows(() => Passport.available());
    t.notThrows(() => Passport.accountWithIdExists('test'));
});

windowsTest('check KeyCreationOption exists on windows', (t) => {
    t.notThrows(() => KeyCreationOption.FailIfExists);
    t.notThrows(() => KeyCreationOption.ReplaceExisting);
});

windowsTest('check PublicKeyEncoding exists on windows', (t) => {
    t.notThrows(() => PublicKeyEncoding.X509SubjectPublicKeyInfo);
    t.notThrows(() => PublicKeyEncoding.Pkcs1RsaPublicKey);
    t.notThrows(() => PublicKeyEncoding.BCryptEccFullPublicKey);
    t.notThrows(() => PublicKeyEncoding.BCryptPublicKey);
    t.notThrows(() => PublicKeyEncoding.Capi1PublicKey);
});

windowsTest('check VerificationResult exists on windows', (t) => {
    t.notThrows(() => VerificationResult.Canceled);
    t.notThrows(() => VerificationResult.Verified);
    t.notThrows(() => VerificationResult.DeviceNotPresent);
    t.notThrows(() => VerificationResult.NotConfiguredForUser);
    t.notThrows(() => VerificationResult.DisabledByPolicy);
    t.notThrows(() => VerificationResult.DeviceBusy);
    t.notThrows(() => VerificationResult.RetriesExhausted);
});

windowsLocalTest('sign and verify', async (t) => {
    t.true(Passport.available());

    t.is(
        await Passport.requestVerification('Please verify your identity'),
        VerificationResult.Verified
    );

    const passport = new Passport('test');
    await passport.createAccount(KeyCreationOption.ReplaceExisting);

    t.true(passport.accountExists);
    t.true(Passport.accountWithIdExists('test'));

    const challenge = randomBytes(32);
    const signature = await passport.sign(challenge);

    const keyBuffer = await passport.getPublicKey();
    const key = createPublicKey({
        key: keyBuffer,
        format: 'der',
        type: 'pkcs1',
    });

    const verify = createVerify('SHA256');
    verify.write(challenge);
    verify.end();

    t.true(verify.verify(key, signature));

    await passport.deleteAccount();
    t.false(passport.accountExists);
    t.false(Passport.accountWithIdExists('test'));
});

unixTest('check Passport exceptions on unix', (t) => {
    t.throws(() => new Passport('test'), MODULE_NOT_FOUND);
    t.throws(() => Passport.accountWithIdExists('test'), MODULE_NOT_FOUND);
    t.throws(() => Passport.requestVerification('test'), MODULE_NOT_FOUND);
    t.notThrows(() => Passport.available());
    t.false(Passport.available());
});

unixTest('check KeyCreationOption exceptions on unix', (t) => {
    t.throws(() => KeyCreationOption.FailIfExists, MODULE_NOT_FOUND);
    t.throws(() => KeyCreationOption.ReplaceExisting, MODULE_NOT_FOUND);
});

unixTest('check PublicKeyEncoding exceptions on unix', (t) => {
    t.throws(
        () => PublicKeyEncoding.X509SubjectPublicKeyInfo,
        MODULE_NOT_FOUND
    );
    t.throws(() => PublicKeyEncoding.Pkcs1RsaPublicKey, MODULE_NOT_FOUND);
    t.throws(() => PublicKeyEncoding.BCryptEccFullPublicKey, MODULE_NOT_FOUND);
    t.throws(() => PublicKeyEncoding.BCryptPublicKey, MODULE_NOT_FOUND);
    t.throws(() => PublicKeyEncoding.Capi1PublicKey, MODULE_NOT_FOUND);
});

unixTest('check VerificationResult exceptions on unix', (t) => {
    t.throws(() => VerificationResult.Canceled, MODULE_NOT_FOUND);
    t.throws(() => VerificationResult.Verified, MODULE_NOT_FOUND);
    t.throws(() => VerificationResult.DeviceNotPresent, MODULE_NOT_FOUND);
    t.throws(() => VerificationResult.NotConfiguredForUser, MODULE_NOT_FOUND);
    t.throws(() => VerificationResult.DisabledByPolicy, MODULE_NOT_FOUND);
    t.throws(() => VerificationResult.DeviceBusy, MODULE_NOT_FOUND);
    t.throws(() => VerificationResult.RetriesExhausted, MODULE_NOT_FOUND);
});
