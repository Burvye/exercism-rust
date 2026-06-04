use std::collections::HashMap;

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut nth = 0;
    let mut maybe_prime = 1 as u32;
    // map of checked numbers to not recheck
    let mut divcounter: HashMap<u32, u32> = HashMap::new();

    while nth <= n {
        maybe_prime += 1;
        let is_prime = update_hash(&mut divcounter, maybe_prime);
        println!(
            "nth is currently {} and {} is a prime: {}",
            nth, maybe_prime, is_prime,
        );
        if is_prime {
            divcounter.insert(maybe_prime, 2 * maybe_prime);
            nth += 1;
        }
    }
    maybe_prime
}
pub fn nth_slow(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut nth = 0;
    let mut maybe_prime = 1 as u32;
    // map of checked numbers to not recheck

    while nth <= n {
        maybe_prime += 1;
        let is_prime = check_if_prime(maybe_prime);
        println!(
            "nth is currently {} and {} is a prime: {}",
            nth, maybe_prime, is_prime,
        );
        if is_prime {
            nth += 1;
        }
    }
    maybe_prime
}
fn update_hash(hash: &mut HashMap<u32, u32>, maybe_prime: u32) -> bool {
    let mut not_found: bool = true;
    // O(n)
    for element in hash {
        if *element.1 == maybe_prime {
            *element.1 = maybe_prime + element.0;
            not_found = false;
        }
    }
    not_found
}
fn check_if_prime(maybe_prime: u32) -> bool {
    let sqrn = maybe_prime.isqrt();
    for check in 2..=sqrn {
        if maybe_prime % check == 0 {
            return false;
        }
    }
    true
}
