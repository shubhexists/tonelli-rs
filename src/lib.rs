pub fn pow_mod(mut x: u64, mut n: u64, p: u64) -> u64 {
    if p == 0 {
        panic!();
    }

    let mut result = 1;
    x %= p;

    while n > 0 {
        if n & 1 == 1 {
            result = (result * x) % p;
        }
        x = (x * x) % p;
        n >>= 1;
    }

    result
}

pub fn legendre_symbol(a: u64, p: u64) -> i32 {
    let a_mod_p = a % p;
    if a_mod_p == 0 {
        return 0;
    }

    let result = pow_mod(a_mod_p, (p - 1) / 2, p);
    if result == 1 {
        1
    } else if result == p - 1 {
        -1
    } else {
        0
    }
}

pub fn find_quadratic_non_residue(p: u64) -> u64 {
    for z in 2..p {
        if legendre_symbol(z, p) == -1 {
            return z;
        }
    }
    unreachable!();
}

pub fn tonelli_shanks(n: u64, p: u64) -> Option<u64> {
    if p == 2 {
        return Some(n % 2);
    }

    if p % 2 == 0 {
        panic!();
    }

    let n_mod_p = n % p;
    if n_mod_p == 0 {
        return Some(0);
    }

    if legendre_symbol(n_mod_p, p) != 1 {
        return None;
    }

    if p % 4 == 3 {
        let r = pow_mod(n_mod_p, (p + 1) / 4, p);
        return Some(r);
    }

    let mut s = 0;
    let mut q = p - 1;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    let z = find_quadratic_non_residue(p);
    let mut c = pow_mod(z, q, p);
    let mut r = pow_mod(n_mod_p, (q + 1) / 2, p);
    let mut t = pow_mod(n_mod_p, q, p);
    let mut m = s;

    while t != 1 {
        let mut tt = t;
        let mut i = 0;

        while tt != 1 {
            tt = (tt * tt) % p;
            i += 1;
            if i == m {
                return None;
            }
        }

        let b = pow_mod(c, 1 << (m - i - 1), p);
        let b2 = (b * b) % p;
        r = (r * b) % p;
        t = (t * b2) % p;
        c = b2;
        m = i;
    }

    Some(r)
}

pub fn square_roots(n: u64, p: u64) -> Option<(u64, u64)> {
    tonelli_shanks(n, p).map(|r| {
        let r2 = p - r;
        if r < r2 { (r, r2) } else { (r2, r) }
    })
}
