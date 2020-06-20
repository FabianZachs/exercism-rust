pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut candidates = 2..;

    while primes.len() <= n as usize {
        let num = candidates.next().unwrap();
        if !primes.iter().any(|&p| num % p == 0) {
            primes.push(num)
        }
    }

    primes[n as usize] as u32
}
