import { createDummy, DummyType } from './dummies';

const KeyCreationOption = createDummy({
    key: 'KeyCreationOption',
    dummies: {
        FailIfExists: DummyType.Getter,
        ReplaceExisting: DummyType.Getter,
    },
});

const PublicKeyEncoding = createDummy({
    key: 'PublicKeyEncoding',
    dummies: {
        X509SubjectPublicKeyInfo: DummyType.Getter,
        BCryptPublicKey: DummyType.Getter,
        Capi1PublicKey: DummyType.Getter,
        BCryptEccFullPublicKey: DummyType.Getter,
        Pkcs1RsaPublicKey: DummyType.Getter,
    },
});

const VerificationResult = createDummy({
    key: 'VerificationResult',
    dummies: {
        Canceled: DummyType.Getter,
        DeviceBusy: DummyType.Getter,
        DeviceNotPresent: DummyType.Getter,
        DisabledByPolicy: DummyType.Getter,
        NotConfiguredForUser: DummyType.Getter,
        RetriesExhausted: DummyType.Getter,
        Verified: DummyType.Getter,
    },
});

const Passport = createDummy({
    key: 'Passport',
    isClass: true,
    dummies: {
        accountWithIdExists: DummyType.Function,
        available: DummyType.Function,
        requestVerification: DummyType.Function,
    },
    overrides: {
        available: () => false,
    },
});

export { KeyCreationOption, PublicKeyEncoding, VerificationResult, Passport };
