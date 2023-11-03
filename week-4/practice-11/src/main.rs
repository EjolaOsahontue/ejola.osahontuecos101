fn main() {
    let a: i32 = 132 - 2;
    let b: i32 = 132 - 3;

    let result_and = a & b;
    println!("(a & b) = {}", result_and);

    let result_or = a | b;
    println!("(a | b) = {}", result_or);

    let result_xor = a ^ b;
    println!("(a ^ b) = {}", result_xor);

    let result_not_b = !b;
    println!("(!b) = {}", result_not_b);

    let result_left_shift = a< b;
    println!("(a << b) = {}", result_left_shift);

    let result_right_shift = a >b;
    println!("(a >> b) = {}", result_right_shift);
}
