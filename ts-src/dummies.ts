type IndexTypes = typeof import('../native');
type IndexKeys = keyof IndexTypes;

export const enum DummyType {
    Getter,
    Function,
}

type OmitPrototype<
    Key extends IndexKeys,
    IsClass extends boolean,
> = IsClass extends true ? Omit<IndexTypes[Key], 'prototype'> : IndexTypes[Key];

type AnyFunction = (...args: any[]) => any;
type OverrideFunction<T> = T extends AnyFunction
    ? (...args: Parameters<T>) => ReturnType<T>
    : AnyFunction;

type DummyRecord<Key extends IndexKeys, IsClass extends boolean, T> = Record<
    keyof OmitPrototype<Key, IsClass>,
    T
>;

type DummyOverride<Key extends IndexKeys, IsClass extends boolean> = {
    [key in keyof OmitPrototype<Key, IsClass>]?: OverrideFunction<
        OmitPrototype<Key, IsClass>[key]
    >;
};

interface DummyOptions<Key extends IndexKeys, IsClass extends boolean> {
    key: Key;
    isClass?: IsClass;
    dummies: DummyRecord<Key, IsClass, DummyType>;
    overrides?: DummyOverride<Key, IsClass>;
}

type DummyOptsKeys<
    Key extends IndexKeys,
    IsClass extends boolean,
> = keyof DummyOptions<Key, IsClass>['dummies'];

export function createDummy<Key extends IndexKeys, IsClass extends boolean>(
    opts: DummyOptions<Key, IsClass>
): Readonly<IndexTypes[Key]> {
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

        return Object.freeze(
            Object.defineProperties(
                obj as IndexTypes[Key],
                (
                    Object.keys(opts.dummies) as DummyOptsKeys<Key, IsClass>[]
                ).reduce((prev, cur) => {
                    const key =
                        opts.dummies[cur] === DummyType.Getter
                            ? 'get'
                            : 'value';

                    return {
                        ...prev,
                        [cur]: {
                            [key]: (
                                ...args: Parameters<IndexTypes[Key][typeof cur]>
                            ) => {
                                if (opts.overrides && opts.overrides[cur]) {
                                    return opts.overrides[cur]!(...args);
                                } else {
                                    throw e;
                                }
                            },
                            enumerable: true,
                        },
                    };
                }, {})
            )
        );
    }
}
