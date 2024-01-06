use std::io;

const MOD: i64 = 0x7fffffffffffffe7; // 216289611853439384th prime
const A01: i64 = 0x7fffffffffffff5b;
const B01: i64 = 0x7ffffffffffffefd;
const SMALL_MOD: i32 = 0x7fffffff; // 105097565th prime
const BIG_MOD: i128 = 0x7fffffffffffffffffffffffffffffff;

fn flr64(a: i64, b: i64) -> i64 {
    if a % b != 0 { a / b - (((a < 0) != (b < 0)) as i64) } else { a / b }
}

fn mod64(a: i64, m: i64) -> i64 {
    a - m * flr64(a, m)
}

fn add64(a: i64, b: i64, m: i64) -> i64 {
    let a = mod64(a, m);
    let b = mod64(b, m);
    if m - a < b {
        if a > b { a - m + b } else { b - m + a }
    } else {
        a + b
    }
}

fn sub64(a: i64, b: i64, m: i64) -> i64 {
    let a = mod64(a, m);
    let b = mod64(b, m);
    if a < b {
        a - b + m
    } else {
        a - b
    }
}

fn mul64(a: i64, b: i64, m: i64) -> i64 {
    let (mut a, mut b) = (mod64(a, m), mod64(b, m));
    if b < 0 {
        b = -b;
        a = -a;
    }
    let mut res = 0;
    while b != 0 {
        if (b & 1) != 0 {
            res = add64(res, a, m);
        }
        a = add64(a, a, m);
        b >>= 1;
    }
    res
}

fn binpow(a: i64, b: i64, m: i64) -> i64 {
    let mut a = mod64(a, m);
    let mut b = b;
    let mut res = 1;
    while b != 0 {
        if (b & 1) != 0 {
            res = mul64(res, a, m);
        }
        a = mul64(a, a, m);
        b >>= 1;
    }
    mod64(res, m)
}

fn div64(a: i64, b: i64, m: i64) -> i64 {
    mul64(a, binpow(b, -1, m), m)
}

fn miller_rabin(n: i64, b: i64) -> bool {
    let mut m = n - 1;
    let mut cnt = 0;
    while m % 2 == 0 {
        m >>= 1;
        cnt += 1;
    }
    let ret = binpow(b, m, n);
    if ret == 1 || ret == n - 1 {
        return true;
    }
    while cnt > 0 {
        let ret = mul64(ret, ret, n);
        if ret == n - 1 {
            return true;
        }
        cnt -= 1;
    }
    false
}

fn ptest(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    const BASIC: [i64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &basic in BASIC.iter().take_while(|&&x| x < n) {
        if !miller_rabin(n, basic) {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut numbers = input.split_whitespace();
    let a: i64 = numbers.next().unwrap().parse().unwrap();
    let b: i64 = numbers.next().unwrap().parse().unwrap();
    let m: i64 = numbers.next().unwrap().parse().unwrap();
    println!("floor({}/{})={}", a, b, flr64(a, b));
    println!("{}={} (mod {})", a, mod64(a, m), m);
    println!("{}={} (mod {})", b, mod64(b, m), m);
    println!("{}+{}={} (mod {})", a, b, add64(a, b, m), m);
    println!("{}-{}={} (mod {})", a, b, sub64(a, b, m), m);
    println!("{}*{}={} (mod {})", a, b, mul64(a, b, m), m);
    println!("{}/{}={} (mod {})", a, b, div64(a, b, m), m);
    println!("{}^{}={} (mod {})", a, b, binpow(a, b, m), m);
}
