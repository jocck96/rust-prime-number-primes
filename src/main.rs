fn sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();
    for p in 2..=limit {
        if is_prime[p] {
            primes.push(p);
            let mut i = p * p;
            while i <= limit {
                is_prime[i] = false;
                i += p;
            }
        }
    }
    primes
}

fn main() {
    println!("Primes up to 50: {:?}", sieve(50));
}
