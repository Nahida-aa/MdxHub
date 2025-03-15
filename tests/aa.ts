const aaConfig = {
    a: 1,
    b: 2
}
type AAConfig = typeof aaConfig

const fun1 = (c: AAConfig) => {
    return c;
}

const fun2 = (c: AAConfig) => {
    return c;
}

type FunType = typeof fun1

const funList: FunType[] = [
    fun1,
    fun2
];

const initItem = aaConfig;
const ret = funList.reduce((prev, item) => item(prev), initItem);

// ===
const ret2 = fun2(fun1(aaConfig));