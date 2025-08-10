# tonelli-rs

Pure Rust implementation of Tonelli Shanks Algorithm used in Elliptic Curve Cryptography used to calculate square roots of numbers in finite fields.

## Limitation

This library uses standard Rust types yet which makes it unsuitable to use it with very big algorithms like `keccak256`. However it works great if the use case is within `u64`.

## Usage 

Refer [examples](https://github.com/shubhexists/tonelli-rs/tree/main/examples)

## Explanation

Although I'm not an expert, this is a very basic high level overview of how the flow is working - 

- Check if n (mod p) is a Quadratic Residue where `n` is the number we need to find square root of and `p` is the order of the finite field we are working with. We check for quadratic residue using `legendre_symbol` which inturn is justified by euler's criterion -

```
(n/p) ≡ n^((p-1)/2) (mod p)
```

If the value of `legendre_symbol` is one, that means n (mod p) has a quadratic residue.

- If the value of `legendre_symbol` is one for n (mod p), then only the square root of that number exists in that particular finite field, otherwise we simply return `None`.

- Check if 
```
p = 3 (mod 4)
```

i.e. p on dividing by 4 leaves remainder 3. If yes, then we can simply calculate the square root by - 

```
Roots -> ((n (mod p) )^ (p+1) /4 ) mod p
```

> Note: Since p is a prime number (p != 2), then only 2 cases are possible. 
> Either `p = 3 (mod 4)` or `p = 1 (mod 4)`. Because `p = 0 (mod 4)` or `p = 2 (mod 4)` would indicate that 2 or 4 would be a factor. Hence number won't be prime anymore.

- To handle the other case i.e. `p = 1 (mod 4)`, we have to first calculate the 1st (smallest) quadratic non-residue. 

> We can essentially take any quadratic non-residue but calculating the smallest one is the fastest and hence most optimal for the calulation. In a field of p, half the values are quadratic residue and the rest other half is quadratic non-residue.

- Calcuate `q` by representing the value of `p - 1` as a multiple of `2^s * q`. 
```
p - 1 = 2^s * q 
```
`q` should be the smallest possible odd value that satisfies this. We'll use this value in calculating the correction value in the next steps.

- Use that quadratic non-residue value to calculate the following values - 
    
    1) `c = z^q mod p`. "c" is the correction value. It is used to calculate the value we'll multiply to "r" to correct the error.
    2) `r = n^((q+1)/2) mod p`. It is our first guess for the square root of the number `n`. 
    3) `t = n^q mod p`. It is the error factor that the first guessed `r` value differs from the actual square root.
    4) `m = s`. Tracks the order

    The end goal is to make `t=1`. The moment `t` (error factor) is one, the value of `r` at that point would be our square root.

- Have a loop till we get `t = 1`. Every iteration, we will calculate the value of another variable `i` such that - 

```
t^(2^i) ≡ 1 (mod p) /// Smallest value of i
```

We will use `c` and `i` to calculate the correction multiplier `b` which we multiply to `r` every iteration to correct the error - 
```
b = c^(2^(m-i-1)) mod p
r = (r × b) mod p
t = (t × b²) mod p  
c = b² mod p
m = i
```

Assume all this as `r^2 = t*n`, so we need to get `t` to 1 ot get `r` as the square root of the number `n` in prime field `p`.

----

The above implementation is referred from [Wikipedia](https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm#The_algorithm) and was origanlly implemented as a result of reading [Article](https://rareskills.io/post/finite-fields#modular-square-roots).

