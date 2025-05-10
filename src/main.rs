fn main() {
    variables();
}

// fn hello_world(){
//     //Con salto de linea
//     println!("Hello, World!");
//     //Sin salto de linea
//     print!("Goodbye, Wowrld!");
// }

fn variables(){
    //Las variables en Rust son inmutables
    let x = 5;
    println!("El valor de la variable x es: {}", x);

    //Si necesitar cambiar el valor de una variable deberas hacerla mutable

    let mut contador = 1;
    contador+=1;
    println!("El valor del contador ahora es: {}", contador);

    //shadowing: Sobrescribir variables

    let x = 10;
    let x = x +10;
    let x = x/2;

    println!("El valor de x es: {}", x)
    
    //mut permite cambiar el valor, shadowing permite cambiar tipo y valor.
}
