import { createDummies, DummyType } from './dummies';

export = createDummies({
    Passport: {
        isClass: true,
        dummies: {
            accountWithIdExists: DummyType.Function,
            available: DummyType.Function,
            requestVerification: DummyType.Function,
        },
        overrides: {
            available: () => false,
        },
    },
    VerificationResult: {
        dummies: {
            Canceled: DummyType.Getter,
            DeviceBusy: DummyType.Getter,
            DeviceNotPresent: DummyType.Getter,
            DisabledByPolicy: DummyType.Getter,
            NotConfiguredForUser: DummyType.Getter,
            RetriesExhausted: DummyType.Getter,
            Verified: DummyType.Getter,
        },
    },
    PublicKeyEncoding: {
        dummies: {
            X509SubjectPublicKeyInfo: DummyType.Getter,
            BCryptPublicKey: DummyType.Getter,
            Capi1PublicKey: DummyType.Getter,
            BCryptEccFullPublicKey: DummyType.Getter,
            Pkcs1RsaPublicKey: DummyType.Getter,
        },
    },
    KeyCreationOption: {
        dummies: {
            FailIfExists: DummyType.Getter,
            ReplaceExisting: DummyType.Getter,
        },
    },
});
