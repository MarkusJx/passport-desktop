import test from 'ava';

import { KeyCreationOption, Passport, VerificationResult } from '../.';
import { createPublicKey, createVerify, randomBytes } from 'crypto';
import isCi from 'is-ci';

test('available', (t) => {
    if (process.platform !== 'win32') {
        t.pass('Skipping test on non-Windows');
        return;
    }

    t.notThrows(() => Passport.available());
    t.notThrows(() => Passport.accountWithIdExists('test'));
});

test('sign and verify', async (t) => {
    if (isCi) {
        t.pass('Skipping test in CI');
        return;
    } else if (process.platform !== 'win32') {
        t.pass('Skipping test on non-Windows');
        return;
    }

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

test('check exceptions', (t) => {
    if (process.platform === 'win32') {
        t.pass('Skipping test on Windows');
        return;
    }

    const MODULE_NOT_FOUND = {
        code: 'MODULE_NOT_FOUND',
        message: /^Cannot find module '.+'$/m,
    };

    t.throws(() => new Passport('test'), MODULE_NOT_FOUND);
    t.throws(() => Passport.accountWithIdExists('test'), MODULE_NOT_FOUND);
    t.throws(() => Passport.requestVerification('test'), MODULE_NOT_FOUND);
});
