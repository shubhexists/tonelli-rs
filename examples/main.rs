use tonelli_rs::{square_roots, tonelli_shanks};

fn main() {
    let n = 2;
    let p = 7;

    if let Some(root) = tonelli_shanks(n, p) {
        println!("Square root: {}", root);
        println!("{}² ≡ {} (mod {})", root, (root * root) % p, p);

        if let Some((r1, r2)) = square_roots(n, p) {
            println!("Both square roots: {} and {}", r1, r2);
            println!("{}² ≡ {}² ≡ {} (mod {})", r1, r2, n, p);
        }
    } else {
        println!("No square root exists");
    }
}
