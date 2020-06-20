//use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //if factors.is_empty() {
    //    return 0;
    //}
    //let factors: Vec<&u32> = factors
    //    .into_iter()
    //    .filter(|x| **x > 0 && **x < limit)
    //    .collect();

    //let mut set: HashSet<u32> = HashSet::new();
    //for i in factors.iter() {
    //    set.extend::<HashSet<u32>>((**i..limit).step_by(**i as usize).collect());
    //}

    //set.iter().sum()
    // functional
    // iter returns a reference, so x in factors here is a ref to a ref
    //let factors: i32 = factors.iter().filter(|&&x| x > 0 && x < limit).collect();
    (1..limit)
        .filter(|x| factors.iter().any(|&factor| factor > 0 && x % factor == 0))
        .sum()
}
