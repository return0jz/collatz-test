fn config() -> u64 {
    let mut inp = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Enter any number to play with the Collatz Conjecture");
        std::process::exit(1)
    });

    inp.trim().parse().unwrap_or_else(|op| {
        eprintln!("Bad input: must be a positive integer");
        std::process::exit(1)
    })
}
fn main() {
    let mut n = config();
    let mut c = 0;
    loop {
        if (n % 2) == 0 {
            println!("{} is even ... go to {}", n, n/2);
            n = n/2;
        } else {
            if n == 1 {
                println!("n has reached 1 ... terminating");
                println!("{} operation(s) have been performed", c);
                std::process::exit(1);
            }
            println!("{} is odd ... go to {}", n, 3*n + 1);
            n = 3*n + 1;
        }
        c += 1;
    }
}
