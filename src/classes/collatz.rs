fn collatz_num_jumps(mut num: u32) -> u32 {
    let mut jumps = 1;

    while num > 1 {
        num = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };

        jumps += 1
    }

    jumps
}

fn main() {

    println("Jumps collatz to {}",collatz_num_jumps(32))
}
