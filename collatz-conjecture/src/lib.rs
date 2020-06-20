pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;

    let mut steps: u64 = 0;

    while n >= 0 {
        n = match n {
            0 => return None,
            1 => return Some(steps),
            n if n % 2 == 0 => n / 2,
            _ => 3 * n + 1,
        };
        steps += 1;
    }

    Some(steps)
}
