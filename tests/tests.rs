use tonelli_rs::*;

#[test]
fn test_pow_mod() {
    assert_eq!(pow_mod(2, 10, 1000), 24);
    assert_eq!(pow_mod(3, 5, 7), 5);
    assert_eq!(pow_mod(2, 0, 7), 1);
    assert_eq!(pow_mod(0, 5, 7), 0);
}

#[test]
fn test_legendre_symbol() {
    assert_eq!(legendre_symbol(1, 7), 1);
    assert_eq!(legendre_symbol(2, 7), 1);
    assert_eq!(legendre_symbol(3, 7), -1);
    assert_eq!(legendre_symbol(4, 7), 1);
    assert_eq!(legendre_symbol(5, 7), -1);
    assert_eq!(legendre_symbol(6, 7), -1);
    assert_eq!(legendre_symbol(0, 7), 0);
}

#[test]
fn test_find_quadratic_non_residue() {
    assert_eq!(find_quadratic_non_residue(7), 3);
    assert_eq!(find_quadratic_non_residue(11), 2);
    assert_eq!(find_quadratic_non_residue(13), 2);
}

#[test]
fn test_tonelli_shanks() {
    assert_eq!(tonelli_shanks(2, 7), Some(4));
    assert_eq!(tonelli_shanks(3, 7), None);
    assert_eq!(tonelli_shanks(4, 7), Some(2));

    assert_eq!(tonelli_shanks(2, 17), Some(6));
    assert_eq!(tonelli_shanks(3, 17), None);
    assert_eq!(tonelli_shanks(9, 17), Some(14));

    assert_eq!(tonelli_shanks(0, 7), Some(0));
    assert_eq!(tonelli_shanks(1, 7), Some(1));
}

#[test]
fn test_square_roots() {
    let roots = square_roots(2, 7);
    assert_eq!(roots, Some((3, 4)));

    assert_eq!(square_roots(3, 17), None);

    let roots = square_roots(2, 17);
    assert_eq!(roots, Some((6, 11)));
    assert_eq!(square_roots(3, 7), None);

    let roots = square_roots(4, 7);
    assert_eq!(roots, Some((2, 5)));
}

#[test]
fn test_large_prime() {
    let p = 1000000007;
    let n = 123456789;

    if legendre_symbol(n, p) == 1 {
        if let Some(r) = tonelli_shanks(n, p) {
            assert_eq!((r * r) % p, n % p);
        }
    }

    let n2 = 4;
    if let Some(r) = tonelli_shanks(n2, p) {
        assert_eq!((r * r) % p, n2 % p);
    }
}
