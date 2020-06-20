pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidates = 2..;

    while n > 1 {
        let factor = candidates.next().unwrap();

        while n % factor == 0 {
            factors.push(factor);
            n = n / factor;
        }
    }

    factors
}
