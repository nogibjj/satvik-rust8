use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    let mut n3: u8 = rng.gen();
    n3 = n3 % 6;
    match n3 {
        1 => println!("One!"),
        // Match several values
        3 | 5 => println!("This is odd"),
        2 | 4 => println!("Very even very cool"),
        0_u8 | 6_u8..=u8::MAX => println!("Very unexpected"),
    }
}
