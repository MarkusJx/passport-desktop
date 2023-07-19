import test from 'ava';

import { KeyCreationOption, Passport, VerificationResult } from '../.';
import { createPublicKey, createVerify, randomBytes } from 'crypto';
import isCi from 'is-ci';

test('available', (t) => {
    t.true(Passport.available());
    t.notThrows(() => Passport.accountWithIdExists('test'));
});

test('sign and verify', async (t) => {
    if (isCi) {
        t.pass('Skipping test in CI');
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
