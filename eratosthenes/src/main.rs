
const NUMS: u32 = 100000;

// Now this is ACTUALLY garbage.
// You can't go past searching for 100,000 numbers because of usize limits since I decided to use an array instead of a vector
// Yeah

fn sieve() -> u32 {
    let mut prime = [true; NUMS as usize];
    prime[0] = false;
    prime[1] = false;

    for i in 2..NUMS {
        let exp = i*i;
        if prime[i as usize] {
            for j in (exp..NUMS).step_by(i as usize) {
                prime[j as usize] = false;
            }
        }
    }
    let mut ct = 0;
    for num in 1..NUMS {
        let is_prime = prime[num as usize];
        if is_prime {
            ct += 1;
        }
    }
    ct
}

fn main() {
    let bench = std::time::Instant::now();
    println!("Found {} primes from 1 to {} in {}ms", sieve(), NUMS, bench.elapsed().as_millis() );
}
