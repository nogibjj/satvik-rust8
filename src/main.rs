use text_io::read;

fn fibo(n: u32) -> u32 {
    if (n == 0) | (n == 1) {
        return 1;
    }
    let mut x: u32 = 1;
    let mut y: u32 = 1;
    let mut ii: u32 = 0;
    loop {
        if ii==n {
            break
        }
        let temp: u32 = x;
        x = y;
        y += temp;
        ii += 1;
    }
    y
}
fn run_samples() {
    println!("{}", fibo(0));
    println!("{}", fibo(1));
    println!("{}", fibo(2));
    println!("{}", fibo(5));
    println!("{}", fibo(10));
}

fn main() {
    run_samples();
    let n: u32 = read!();
    println!("your input: {}, output: {}", n, fibo(n));
}