fn main() {
    let bunnies: i32 = 4;
    let mut changer = 5;
    println!("{}, {}", bunnies, changer);
    changer = 6;

    const WARP_FACTOR: f64 = 9.9;
    println!("{}, {}", changer, WARP_FACTOR);
    println!("Hello, world!");
}
