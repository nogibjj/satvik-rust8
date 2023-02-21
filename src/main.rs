use text_io::read;

struct  FailU32 {
    number: u32,
    fail: bool,
}

fn pnc(x: u32, y: u32) -> FailU32 {
    if y > x {
        return FailU32{number:0, fail:true};
    }
    let mut numerator: u32 = 1;
    let mut denominator: u32 = 1;
    let mut ii: u32 = 0;
    loop {
        if ii == (x-y){
            break;
        }
        ii += 1;
        numerator *= (x-y+ii-1);
        denominator *= ii;
    }
    FailU32{number:numerator/denominator, fail:false}
}
fn main() {
    println!("What is the number of total elements?");
    let x: u32 = read!();
    println!("What is the number of elements to choose?");
    let y: u32 = read!();
    let answer: FailU32 = pnc(x,y);
    if answer.fail {
        println!("Improper input")
    } else {
        println!("The Answer is {}", answer.number);
    }
}