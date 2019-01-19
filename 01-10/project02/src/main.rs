fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n -2)
    }
}

fn main() {

    let mut sum = 0;
    let mut n = 1;

    while fibonacci(n) < 4000001 {
        if fibonacci(n) % 2 == 0 {
            sum += fibonacci(n);
        }

        n += 1;
    }

    println!("{}", sum);
}
