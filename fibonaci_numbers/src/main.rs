fn main() {
    assert_eq!(fibonaci_loop(2),fibonaci_recurse(2));
    assert_eq!(fibonaci_loop(3),fibonaci_recurse(3));
    assert_eq!(fibonaci_loop(4),fibonaci_recurse(4));
    assert_eq!(fibonaci_loop(8),fibonaci_recurse(8));
    assert_eq!(fibonaci_loop(10),fibonaci_recurse(10));
    assert_eq!(fibonaci_loop(18),fibonaci_recurse(18));
    assert_eq!(fibonaci_loop(25),fibonaci_recurse(25));
}

fn fibonaci_loop(nthnumber: u32) -> u32 {
    let mut f0: u32 = 0;
    let mut f1: u32 = 1;
    let mut res: u32 = 1;
    for _ in 1..nthnumber {
        res = f0 + f1;
        f0 = f1;
        f1 = res;
    }
    res
}

fn fibonaci_recurse(nthnumber: u32) -> u32 {
    if nthnumber <= 0 {0} else if nthnumber == 1 {1} else {fibonaci_recurse(nthnumber-1) + fibonaci_recurse(nthnumber-2)}
}