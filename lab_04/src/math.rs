type N = u128;

const PRIMES: [u128; 46] = [
    3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211,
];

const TESTS: u128 = 2;

fn fermat_test(num: N) -> bool {
    for t in (0..TESTS).map(|i| PRIMES[((num + i) % PRIMES.len() as u128) as usize]) {
        if power(t, num - 1, num) != 1 {
            return false;
        }
    }
    true
}

pub(crate) fn is_prime(num: N) -> bool {
    if num % 2 == 0 {
        return false;
    }

    if !fermat_test(num) {
        return false;
    }

    let limit = f64::ceil(f64::sqrt(num as f64)) as N + 1;
    for i in (3..limit).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub(crate) fn gcd(a: N, b: N) -> N {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub(crate) fn gcd_ext(a: N, b: N) -> (i128, i128, N) {
    if b == 0 {
        (1, 0, a)
    } else {
        let (x, y, d) = gcd_ext(b, a % b);
        (y, x - y * (a / b) as i128, d)
    }
}

pub(crate) fn rev(num: N, field_mod: N) -> N {
    let (res, _, _) = gcd_ext(num, field_mod);
    (res + field_mod as i128) as u128 % field_mod
}

pub(crate) fn power(mut num: N, mut degree: N, field_mod: N) -> N {
    let mut res = 1;
    while degree != 0 {
        if degree & 1 == 1 {
            res = (res * num) % field_mod;
        }
        degree = degree.overflowing_shr(1).0;
        num = (num * num) % field_mod;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_rev() {
        let field_mod = 7823;
        for i in 0..field_mod {
            if gcd(field_mod, i) == 1 {
                let revn = rev(i, field_mod);
                assert_eq!((i * revn) % field_mod, 1);
            }
        }
    }

    #[test]
    fn check_power() {
        assert_eq!(power(17, 7817, 7823), 894);
    }

    // very long test (~15s)
    //#[test]
    //fn check_primality() {
    //assert_eq!(true, is_prime(11021576266998122587));
    //}
}
