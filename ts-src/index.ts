import { createDummy } from './dummies';

const KeyCreationOption = createDummy({
    key: 'KeyCreationOption',
    dummies: {
        FailIfExists: 'getter',
        ReplaceExisting: 'getter',
    },
});

const PublicKeyEncoding = createDummy({
    key: 'PublicKeyEncoding',
    dummies: {
        X509SubjectPublicKeyInfo: 'getter',
        BCryptPublicKey: 'getter',
        Capi1PublicKey: 'getter',
        BCryptEccFullPublicKey: 'getter',
        Pkcs1RsaPublicKey: 'getter',
    },
});

const VerificationResult = createDummy({
    key: 'VerificationResult',
    dummies: {
        Canceled: 'getter',
        DeviceBusy: 'getter',
        DeviceNotPresent: 'getter',
        DisabledByPolicy: 'getter',
        NotConfiguredForUser: 'getter',
        RetriesExhausted: 'getter',
        Verified: 'getter',
    },
});

const Passport = createDummy({
    key: 'Passport',
    isClass: true,
    dummies: {
        accountWithIdExists: 'function',
        available: 'function',
        requestVerification: 'function',
    },
});

export { KeyCreationOption, PublicKeyEncoding, VerificationResult, Passport };
