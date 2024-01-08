use std::io;

const MOD: i64 = 0x7fffffffffffffe7; // 216289611853439384th prime
const A01: i64 = 0x7fffffffffffff5b;
const B01: i64 = 0x7ffffffffffffefd;
const SMALL_MOD: i32 = 0x7fffffff; // 105097565th prime
const BIG_MOD: i128 = 0x7fffffffffffffffffffffffffffffff;

// MOD = 16^^7fffffffffffffe7
// A01 = 16^^7fffffffffffff5b
// B01 = 16^^7ffffffffffffefd

fn flr64(a: i64, b: i64) -> i64 {
    if a % b != 0 && (a < 0) != (b < 0) { a / b - 1 } else { a / b }
}

fn mod64(a: i64, m: i64) -> i64 {
    if (a < 0) == (m < 0) { a % m } else { (a % m) + m }
}

fn add64(a: i64, b: i64, m: i64) -> i64 {
    let a = mod64(a, m);
    let b = mod64(b, m);
    if (0 < m) == (b < m - a) {
        a + b
    } else {
        if (0 < m) == (a < b) { b - m + a } else { a - m + b }
    }
}

fn sub64(a: i64, b: i64, m: i64) -> i64 {
    add64(a, -b, m)
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

fn gcd64(a: i64, b: i64) -> i64 {
    if a % b == 0 { b } else { gcd64(b, a % b) }
}

// fn euler_phi(n: i64) -> i64 {
//     let mut ans = n;
//     let mut i = 2;
//     let mut n = n;
//     while i * i <= n {
//         if n % i == 0 {
//             ans = (ans / i) * (i - 1);
//             while n % i == 0 {
//                 n /= i;
//             }
//         }
//         i += 1;
//     }
//     if n > 1 {
//         ans = (ans / n) * (n - 1);
//     }
//     ans
// }

fn pow64(a: i64, b: i64, m: i64) -> i64 {
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
    mul64(a, pow64(b, mod64(-1, m - 1), m), m)
}

fn miller_rabin(n: i64, b: i64) -> bool {
    let mut m = n - 1;
    let mut cnt = 0;
    while m % 2 == 0 {
        m >>= 1;
        cnt += 1;
    }
    let mut ret = pow64(b, m, n);
    loop {
        if ret == 1 || ret == n - 1 {
            return true;
        }
        ret = mul64(ret, ret, n);
        cnt -= 1;
        if cnt <= 0 {
            break;
        }
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
    println!("{}^{}={} (mod {})", a, b, pow64(a, b, m), m);

    // let mut cnt = 0;
    // for i in 1..100000 {
    //     if ptest(i) {
    //         cnt += 1;
    //     }
    // }
    // println!("{}", cnt);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flr64() {
        assert_eq!(flr64(MOD, A01), 1, "Error");
        assert_eq!(flr64(A01, B01), 1, "Error");
        assert_eq!(flr64(B01, MOD), 0, "Error");
        assert_eq!(flr64(-MOD, A01), -2, "Error");
        assert_eq!(flr64(-A01, B01), -2, "Error");
        assert_eq!(flr64(-B01, MOD), -1, "Error");
        assert_eq!(flr64(MOD, -A01), -2, "Error");
        assert_eq!(flr64(A01, -B01), -2, "Error");
        assert_eq!(flr64(B01, -MOD), -1, "Error");
    }

    #[test]
    fn test_mod64() {
        assert_eq!(mod64(10, 3), 1, "Error");
        assert_eq!(mod64(15, 7), 1, "Error");
        assert_eq!(mod64(-8, 5), 2, "Error");
        assert_eq!(mod64(-11, 4), 1, "Error");
        assert_eq!(mod64(20, -6), -4, "Error");
        assert_eq!(mod64(MOD, A01), 140, "Error");
        assert_eq!(mod64(A01, -MOD), -140, "Error");
        assert_eq!(mod64(-A01, MOD), 140, "Error");
        assert_eq!(mod64(-MOD, -A01), -140, "Error");
        assert_eq!(mod64(MOD, B01), 234, "Error");
        assert_eq!(mod64(B01, -MOD), -234, "Error");
        assert_eq!(mod64(-B01, MOD), 234, "Error");
        assert_eq!(mod64(-MOD, -B01), -234, "Error");
        assert_eq!(mod64(-A01, B01), 9223372036854775455, "Error");
    }

    #[test]
    fn test_add64() {
        assert_eq!(add64(A01, B01, MOD), 9223372036854775409, "Error");
        assert_eq!(add64(-A01, -B01, -MOD), -9223372036854775409, "Error");
        assert_eq!(add64(-A01, -B01, MOD), 374, "Error");
        assert_eq!(add64(A01, -B01, -MOD), -9223372036854775689, "Error");
        assert_eq!(add64(-A01, B01, -MOD), -94, "Error");
        assert_eq!(add64(MOD, B01, A01), 46, "Error");
        assert_eq!(add64(MOD, -B01, A01), 234, "Error");
        assert_eq!(add64(MOD, A01, B01), 328, "Error");
        assert_eq!(add64(-A01, B01, -MOD), -94, "Error");
        assert_eq!(add64(A01, -B01, MOD), 94, "Error");
        assert_eq!(add64(MOD, -A01, B01), 140, "Error");
    }

    #[test]
    fn test_sub64() {
        assert_eq!(sub64(A01, B01, MOD), 94, "Error");
        assert_eq!(sub64(B01, A01, MOD), 9223372036854775689, "Error");
        assert_eq!(sub64(MOD, A01, B01), 140, "Error");
        assert_eq!(sub64(MOD, B01, A01), 234, "Error");
    }

    #[test]
    fn test_mul64() {
        assert_eq!(mul64(0, 0, MOD), 0, "Error");
        assert_eq!(mul64(A01, B01, MOD), 32760, "Error");
        assert_eq!(mul64(-A01, B01, MOD), 9223372036854743023, "Error");
        assert_eq!(mul64(A01, -B01, MOD), 9223372036854743023, "Error");
        assert_eq!(mul64(A01, B01, MOD), 32760, "Error");
        assert_eq!(mul64(A01, B01, -MOD), -9223372036854743023, "Error");
        assert_eq!(mul64(-A01, -B01, -MOD), -9223372036854743023, "Error");
    }

    #[test]
    fn test_gcd64() {
        assert_eq!(gcd64(A01, B01), 1, "Error");
        assert_eq!(gcd64(A01, MOD), 1, "Error");
        assert_eq!(gcd64(B01, MOD), 1, "Error");
        assert_eq!(gcd64(114514, 1919810), 2, "Error");
    }

    #[test]
    fn test_div64() {
        assert_eq!(div64(A01, B01, MOD), 867154635943611399, "Error");
        assert_eq!(div64(B01, A01, MOD), 1449387034362893339, "Error");
        assert_eq!(div64(MOD, B01, A01), 8830888120392870295, "Error");
        assert_eq!(div64(MOD, A01, B01), 392483916461905345, "Error");
        assert_eq!(div64(-A01, B01, MOD), 8356217400911164384, "Error");
        assert_eq!(div64(-B01, A01, MOD), 7773985002491882444, "Error");
        assert_eq!(div64(-MOD, B01, A01), 392483916461905348, "Error");
        assert_eq!(div64(-MOD, A01, B01), 8830888120392870204, "Error");
    }

    #[test]
    fn test_ptest() {
        assert!(ptest(2), "Error");
        assert!(ptest(A01), "Error");
        assert!(!ptest(A01 - 1), "Error");
        assert!(ptest(B01), "Error");
        assert!(!ptest(B01 - 1), "Error");
        assert!(ptest(MOD), "Error");
        assert!(!ptest(MOD - 1), "Error");
    }
}
