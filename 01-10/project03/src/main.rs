//returns true if a number is prime
fn is_prime(n: u64) -> bool {

    let mut p = true;

    let m = (n as f64).sqrt() as u64 + 1;

    for i in 2..m {
        if n % i == 0 {
            p = false
        }
    }

    p
}

fn main() {

    let mut factors: Vec<u64> = Vec::new();
    let mut number = 600851475143;
    let mut lowest_factor = 2;

    while !is_prime(number) {
        if number % lowest_factor == 0 {
            number /= lowest_factor;
            factors.push(lowest_factor);
        }

        lowest_factor += 1
    }

    factors.push(number);

    for i in &factors {
        println!("{}", i);
    }
}
