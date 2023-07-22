type IndexTypes = typeof import('../native');
type IndexKeys = keyof IndexTypes;
type DummyType = 'getter' | 'function';

type OmitPrototype<T extends IndexKeys, C extends boolean> = C extends true
    ? Omit<IndexTypes[T], 'prototype'>
    : IndexTypes[T];

interface DummyOptions<T extends IndexKeys, C extends boolean> {
    key: T;
    isClass?: C;
    dummies: Record<keyof OmitPrototype<T, C>, DummyType>;
}

export function createDummy<T extends IndexKeys, C extends boolean>(
    opts: DummyOptions<T, C>
): IndexTypes[T] {
    try {
        return require('../native')[opts.key];
    } catch (e) {
        const obj = opts.isClass
            ? class {
                  constructor() {
                      throw e;
                  }
              }
            : {};

        return Object.defineProperties(
            obj,
            Object.keys(opts.dummies).reduce((prev, cur) => {
                let val;
                if (
                    opts.dummies[cur as keyof DummyOptions<T, C>['dummies']] ===
                    'getter'
                ) {
                    val = {
                        get: () => {
                            throw e;
                        },
                    };
                } else {
                    val = {
                        value: () => {
                            throw e;
                        },
                    };
                }

                return {
                    ...prev,
                    [cur]: {
                        ...val,
                        enumerable: true,
                    },
                };
            }, {})
        ) as IndexTypes[T];
    }
}
