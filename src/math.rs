pub fn find_coprime(num: i32) -> i32 {
    let mut r = f64::from(num).sqrt() as i32;

    while gcd(r, num) != 1 {
        r += 1;
    }

    r
}

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

pub fn mod_inv(num: i32, modulo: i32) -> i32 {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = modulo;
    let mut new_r = num.abs();

    while new_r != 0 {
        let q = r / new_r;

        let last_t = t;
        let last_r = r;
        t = new_t;
        r = new_r;

        new_t = last_t - (q * new_t);
        new_r = last_r - (q * new_r);
    }

    if r != 1 {
        panic!();
    }

    if t < 0 {
        t += modulo;
    }

    if num < 0 {
        -t
    } else {
        t
    }
}
