//! Tonelli-Shanks algorithm implementation for computing square roots modulo a prime.
//!
//! This library provides functions to compute square roots in the finite field Z/pZ
//! where p is an odd prime number.

/// Computes modular exponentiation: x^n mod p
///
/// Uses the square-and-multiply algorithm for efficient computation.
///
/// # Arguments
/// * `x` - The base
/// * `n` - The exponent
/// * `p` - The modulus (must be positive)
///
/// # Returns
/// The result of x^n mod p
///
/// # Examples
/// ```
/// use tonelli_rs::pow_mod;
///
/// assert_eq!(pow_mod(2, 10, 1000), 24);
/// assert_eq!(pow_mod(3, 5, 7), 5);
/// ```
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

/// Computes the Legendre symbol (a/p)
///
/// The Legendre symbol indicates whether a is a quadratic residue modulo p:
/// * 1 if a is a quadratic residue modulo p
/// * -1 if a is a quadratic non-residue modulo p  
/// * 0 if a ≡ 0 (mod p)
///
/// # Arguments
/// * `a` - The number to check
/// * `p` - The prime modulus
///
/// # Returns
/// The Legendre symbol as an i32
///
/// # Examples
/// ```
/// use tonelli_rs::legendre_symbol;
///
/// assert_eq!(legendre_symbol(2, 7), 1);  // 2 is a quadratic residue mod 7
/// assert_eq!(legendre_symbol(3, 7), -1); // 3 is a quadratic non-residue mod 7
/// ```
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

/// Finds the first quadratic non-residue modulo p
///
/// This function searches for the smallest positive integer z such that
/// z is a quadratic non-residue modulo p.
///
/// # Arguments
/// * `p` - The prime modulus
///
/// # Returns
/// The smallest quadratic non-residue modulo p
///
/// # Examples
/// ```
/// use tonelli_rs::find_quadratic_non_residue;
///
/// assert_eq!(find_quadratic_non_residue(7), 3);
/// ```
pub fn find_quadratic_non_residue(p: u64) -> u64 {
    for z in 2..p {
        if legendre_symbol(z, p) == -1 {
            return z;
        }
    }
    unreachable!();
}

/// Computes a square root of n modulo p using the Tonelli-Shanks algorithm
///
/// This function finds r such that r² ≡ n (mod p) if n is a quadratic residue.
///
/// # Arguments
/// * `n` - The number to find the square root of
/// * `p` - The prime modulus
///
/// # Returns
/// * `Some(r)` if n is a quadratic residue modulo p, where r² ≡ n (mod p)
/// * `None` if n is not a quadratic residue modulo p
///
/// # Examples
/// ```
/// use tonelli_rs::tonelli_shanks;
///
/// // 2 is a quadratic residue modulo 7: 4² ≡ 2 (mod 7)
/// assert_eq!(tonelli_shanks(2, 7), Some(4));
///
/// // 3 is not a quadratic residue modulo 7
/// assert_eq!(tonelli_shanks(3, 7), None);
/// ```
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

/// Computes both square roots of n modulo p
///
/// If n is a quadratic residue modulo p, this function returns both square roots.
///
/// # Arguments
/// * `n` - The number to find the square roots of
/// * `p` - The prime modulus
///
/// # Returns
/// * `Some((r1, r2))` if n is a quadratic residue, where r1 and r2 are the two square roots
/// * `None` if n is not a quadratic residue
///
/// # Examples
/// ```
/// use tonelli_rs::square_roots;
///
/// let roots = square_roots(2, 7);
/// assert_eq!(roots, Some((3, 4))); // 3² ≡ 4² ≡ 2 (mod 7)
/// ```
pub fn square_roots(n: u64, p: u64) -> Option<(u64, u64)> {
    tonelli_shanks(n, p).map(|r| {
        let r2 = p - r;
        if r < r2 { (r, r2) } else { (r2, r) }
    })
}
