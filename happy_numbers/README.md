## happy_numbers
https://www.geeksforgeeks.org/happy-number/ or https://github.com/karan/Projects

```rust
/*
Happy Numbers - A happy number is defined by the following process.

Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay),
or it loops endlessly in a cycle which does not include 1.
Those numbers for which this process ends in 1 are happy numbers, while those that do not end in 1 are unhappy numbers.
https://www.geeksforgeeks.org/happy-number/ or https://github.com/karan/Projects
*/

// Really inefficient way to go about this algorithm.
// If you wanted to make it faster, replace get_digits with a function to return the sum of all of those digits directly,
// Ditching the vector middleman which wastes memory anyway.

static MAX_STACK: u16 = 10;

// Returns a vector of digits of a u32 number.
fn get_digits(num: u32) -> Vec<u8> {
    let mut nums = Vec::<u8>::new();
    let mut current_num = num;
    while current_num != 0 {
        nums.push( (current_num % 10) as u8 );
        current_num /= 10;
    }
    nums
}

fn is_happy_number(num: u32, stack_lvl: u16) -> bool {
    let digits = get_digits(num);
    let mut digit_sum = 0 as u32;
    for digit in &digits {
        digit_sum += digit.pow(2) as u32;
    }
    if digit_sum == 1 {
        return true;
    }else{
        if stack_lvl+1 > MAX_STACK { return false };
        return is_happy_number(digit_sum, stack_lvl+1);
    }
}

// Returns a range of happy numbers between the start and end number.
fn get_happy_range(start_num: u32, end_num: u32) -> Vec<u32> {
    let mut happy_nums = Vec::<u32>::new();
    for num in start_num..end_num {
        if is_happy_number(num, 0) {
            happy_nums.push(num);
        }
    }
    happy_nums
}

fn main() {
    let (min, max) = (1, 5_000_000);
    let bench = std::time::Instant::now();
    println!("Found {} happy numbers in the range of {} to {} in {}s", get_happy_range(min, max).len(), min, max, bench.elapsed().as_secs());
}

```