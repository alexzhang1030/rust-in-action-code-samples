pub fn main() {
    let a = 10;
    let b: i32 = 10;
    let c = 10_i32;
    let d = 10i32;
    let e = add(add(a, b), add(c, d));

    println!("( {a} + {b} ) + ( {c} + {d} ) = {}", e);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
