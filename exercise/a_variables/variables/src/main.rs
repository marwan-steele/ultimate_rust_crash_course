const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // Rust warns about unused variables. Use prefix _unused if intentional
    // let unused = 0;
    // Rust does not allow assignment to const
    // STARTING_MISSILES = 2;

    let (missiles, ready) : (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles-ready );
}
