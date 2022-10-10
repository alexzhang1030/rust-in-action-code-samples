pub fn main() {
    let a = 10;
    let b: i32 = 10;
    let c = 10_i32;
    let d = 10i32;
    let e = add(add(a, b), add(c, d));
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    println!("( {a} + {b} ) + ( {c} + {d} ) = {}", e);

    let binary = 0b11;
    let octal = 0o66;
    let hexadecimal = 0x12C;

    println!("{binary} - {octal} - {hexadecimal}")
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
