fn main() {
    //chars...
    //dado que rust tiene por defecto la configuracion utf-8 podemos utilizar emojis y simbolos sin tener que hacer nada!
    let letra = 'a'; //se usan comillas simples para caracteres
    let emoji = '😎';
    let simbolo = '∑';

    println!("Letras y símbolos: {}, {}, {}", letra, emoji, simbolo);

    //booleans...
    //Como en cualquier lenguaje de programacion, true o false, sirve tanto para bucles como para estructuras condicionales

    //un ejemplo de uso de booleanos muy basico:

    let mayor_de_edad = true;


    if mayor_de_edad{
        println!("Eres mayor de edad!");
    }
    else{
        println!("Aun no eres mayor de edad!");
    }

    //En Rust, Unit type se refiere a un valor de retorno vacio, es decir, una ausencia de valor
    //Este es el valor por defecto de aquellas funciones que no poseen un return

    println!("Tipo de unit: {:?}", no_retorno_nada());

}

fn no_retorno_nada(){
    println!("No retorno nada! ");
}