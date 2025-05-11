fn main() {
    rangos_numericos()
}

fn rangos_numericos(){
    /*
    En rust existen distintos rangos para numeros dependiendo su uso, esto esta relacionado con los registros de la cpu y se manejan en base binaria 
    */
    println!("\nRangos:");
    println!("i8  => de {} a {}", std::i8::MIN, std::i8::MAX);
    println!("u8  => de {} a {}", std::u8::MIN, std::u8::MAX);
    println!("i16 => de {} a {}", std::i16::MIN, std::i16::MAX);
    println!("u16 => de {} a {}", std::u16::MIN, std::u16::MAX);
    println!("i32 => de {} a {}", std::i32::MIN, std::i32::MAX);
    println!("u32 => de {} a {}", std::u32::MIN, std::u32::MAX);
    println!("f32 => de {} a {}", std::f32::MIN, std::f32::MAX);
    
    //El codigo que estas viendo imprime los rangos numericos de cada uno de los tipos numericos:
    // i8  => de -128 a 127
    // u8  => de 0 a 255
    // i16 => de -32768 a 32767
    // u16 => de 0 a 65535
    // i32 => de -2147483648 a 2147483647
    // u32 => de 0 a 4294967295
    // f32 => de 1.17549435e-38 a 3.40282347e+38

    //Tambien podemos usar otras bases numericas, asi como binarios, octales o hexadecimales

    // Decimal (base 10) - estándar
    let decimal: u8 = 100;
    println!("Decimal: {}", decimal);

    // Binario (base 2) - prefijo 0b
    let binario: u8 = 0b0110_0100; // 100 en binario
    println!("Binario: {}", binario);

    // Octal (base 8) - prefijo 0o
    let octal: u8 = 0o144; // 100 en octal
    println!("Octal: {}", octal);

    // Hexadecimal (base 16) - prefijo 0x
    let hexadecimal: u8 = 0x64; // 100 en hexadecimal
    println!("Hexadecimal: {}", hexadecimal);

    //salidas:

    // Decimal: 100
    // Binario: 100
    // Octal: 100
    // Hexadecimal: 100

    //validaciones con distintas bases 

    assert!(5 < 10);      //Verdadero
    assert!(10 > 4);      //Verdadero

    assert_eq!(2+2, 4);   //Verdadero
    assert_ne!(3 * 3, 10);//Verdadero

    //si fallan, hay excepcion

    //Rangos numericos

    for i in 1..5 { //excluye al 5
        println!("{}", i);
        /*
            Output:
            1
            2
            3
            4
        */
    }

    for i in 1..=5 { //incluye al 5
        println!("{}", i);
            /*
            Output:
            1
            2
            3
            4
            5
        */
    }

    //Rangos con caracteres:
    for c in 'a'..='f'{
        println!("{}", c);
            /*
            Output:
            a
            b
            c
            d
            e
            f
        */
    }

    //Conversion entre tipos usando "as"

    let x: u8 = 20;
    let y = x as u32;
    println!("{}, {}", x, y);

    //overflow

    let x: u8 = 255;
    let y = x.wrapping_add(1); // -> 0, es como que da la vuelta y evita el error de overflow
    
    println!("{}, {}", x, y);

    //logica booleana

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    //Operaciones Bitwise (bit a bit)
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    //se puede usar decodaroes para mejorar la legibilidad de los numeros 
    println!("un millon se puede escribir como: {}", 1_000_000u32);
 }