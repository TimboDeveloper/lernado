fn main() {

    // Integer Types
    // 8-Bit  i8 
    // 16-Bit i16
    // 32-Bit i32
    // 64-Bit i64
    // 128-Bit i128
    // arch isize

    // the isize will get the computer architecture and define 32-Bit or 64-Bit

    // Unsigned Integer Types
    // 8-Bit  u8 
    // 16-Bit u16
    // 32-Bit u32
    // 64-Bit u64
    // 128-Bit u128
    // arch usize

    let common_decimal: i32 = 98222;
    let decimal_with_underscore: i32 = 98_222;
    let hex: i32 = 0xff;
    let octal: i32 = 0o77;
    let common_binary: i32 = 0b11110000;
    let binary_with_underscore: i32 = 0b1111_0000;
    let byte: u8 = b'A'; // Equal to char of C

    println!("");
    println!("Integers");
    println!("");
    println!("Decimal: {}", common_decimal);
    println!("Decimal with underscore: {}", decimal_with_underscore);
    println!("Hex: {}", hex);
    println!("Octal: {}", octal);
    println!("Binary: {}", common_binary);
    println!("Binary with underscore: {}", binary_with_underscore);
    println!("Byte: {}", byte);

    // By default rust use f64 for float numbers

    let float_64_bits = 2.0;
    let float_32_bits: f32 = 2.0;

    let float_32_bits_precision: f32 = 0.1 + 0.2;
    let float_32_bits_parameter: f32 = 0.3;

    println!("");
    println!("Floats");
    println!("");
    println!("Float f64: {}", float_64_bits);
    println!("Float f32: {}", float_32_bits);
    println!("Equality of floats type: float_64_bits == float_32_bits -> {}", float_64_bits == float_32_bits);
    println!("Float f64 precision equality: 0.1 + 0.2 == 0.3 -> {}", 0.1 + 0.2 == 0.3);
    // I don't know yet but f32 solves the little mess of approximate numbers to this calculation
    println!("Float f32 precision equality: 0.1 + 0.2 == 0.3 -> {}", float_32_bits_precision == float_32_bits_parameter);
}
