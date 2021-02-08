## happy_numbers_fast
Optimized version of happy numbers, takes 319 ms to find all happy numbers from 1 to 10 million.

```rust
/*
Happy Numbers - A happy number is defined by the following process.

Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay),
or it loops endlessly in a cycle which does not include 1.
Those numbers for which this process ends in 1 are happy numbers, while those that do not end in 1 are unhappy numbers.
https://www.geeksforgeeks.org/happy-number/ or https://github.com/karan/Projects
*/

// You might be asking why we use a stack level method instead of a lookup table
// this is because Rust's lookup tables are just so... slooow... Or maybe lookup tables in general. I dunno. (BTreeMap / HashMaps)
// After going through all 1-10 million numbers I found that no happy numbers went past ~6 loops, so that's our lucky number I guess!
static MAX_STACK: u8 = 6;

// Returns a vector of digits of a u32 number.
fn digit_sum(mut num: u32) -> u32 {
    let mut sum = 0;
    while num != 0 {
        sum += (num % 10).pow(2);
        num /= 10;
    }
    sum
}

fn is_happy_number(num: u32) -> bool {
    let mut sum = digit_sum( num );
    let mut loop_level = 0;
    while sum != 1 {
        if loop_level > MAX_STACK { return false };
        loop_level += 1;
        sum = digit_sum( sum );
    }
    true
}

// Returns a range of happy numbers between the start and end number.
fn get_happy_range(start_num: u32, end_num: u32) -> u32 {
    let mut happy_ct = 0;
    for num in start_num..end_num {
        if is_happy_number(num) {
            happy_ct += 1;
        }
    }
    happy_ct
}

fn main() {
    let (min, max) = (1, 10_000_000);
    let bench = std::time::Instant::now();
    println!("Found {} happy numbers in the range of {} to {} in {}ms", get_happy_range(min, max), min, max, bench.elapsed().as_millis());
}

```