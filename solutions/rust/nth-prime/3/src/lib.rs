use std::collections::HashMap;

pub fn nth_slowest(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut nth = 0;
    let mut maybe_prime = 1 as u32;
    let mut divcounter: HashMap<u32, u32> = HashMap::new();

    while nth <= n {
        maybe_prime += 1;
        let is_prime = update_hash(&mut divcounter, maybe_prime);
        if is_prime {
            divcounter.insert(maybe_prime, 2 * maybe_prime);
            nth += 1;
        }
    }
    maybe_prime
}
pub fn nth(n: u32) -> u32 {
    let mut composites: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut count = 0;
    for candidate in 2.. {
        match composites.remove(&candidate) {
            None => {
                if count == n {
                    return candidate;
                }
                count += 1;
                let Some(square) = candidate.checked_mul(candidate) else {
                    continue;
                };
                composites.entry(square).or_default().push(candidate);
            }
            Some(primes) => {
                for prime in primes {
                    let next = candidate + prime;
                    composites.entry(next).or_default().push(prime);
                }
            }
        }
    }
    unreachable!()
}
pub fn nth_slow(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut nth = 0;
    let mut maybe_prime = 1 as u32;
    let mut primes = Vec::new();
    while nth <= n {
        maybe_prime += 1;
        let is_prime = is_prime_iterator(&primes, maybe_prime);
        if is_prime {
            nth += 1;
            primes.push(maybe_prime);
        }
    }
    maybe_prime
}
pub fn nth_naive(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut nth = 0;
    let mut maybe_prime = 1 as u32;
    // map of checked numbers to not recheck

    while nth <= n {
        maybe_prime += 1;
        let is_prime = check_if_prime(maybe_prime);
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

fn is_prime(primes: &[u32], maybe_prime: u32) -> bool {
    let mut not_found = true;
    let sqrn = maybe_prime.isqrt();
    for element in primes {
        if *element > sqrn {
            break;
        }
        if maybe_prime % *element == 0 {
            not_found = false;
            break;
        }
    }
    not_found
}
fn is_prime_iterator(primes: &[u32], maybe_prime: u32) -> bool {
    let sqrn = maybe_prime.isqrt();
    primes
        .iter()
        .take_while(|element| element <= &&sqrn)
        .all(|element| maybe_prime % element != 0)
}
