use std::io::{
    stdin, stdout, Write
};
mod calc;



fn read(input: &mut String) {
    stdout().flush().expect("flush failed!");
    stdin().read_line(input).expect("read failed!");
}

fn main() {
    let mut ix = String::new();
    let mut iy = String::new();
    let mut op = String::new();
    let operators = String::from("+-*/v");

    println!("Bienvenido a tu calculadora!!");
    println!("--------------------------------");
    println!("¿Qué operación quieres realicar?");
    println!("Opciones: más(+), menos(-), multiplicaciòn(*), divición(/) y raiz cuadrada(v) ");

    read(&mut op);
    let op: char = op.trim().chars().next().unwrap();
    if !operators.contains(op) {
        println!("Operación desconocida");
        return;
    }

    println!("valor:");
    read(&mut ix);
    

    if op != 'v' {
        println!("segundo valor:");
        read(&mut iy);
    } else {
        iy = String::from("0");
    }

    let x: i32 = ix.trim().parse().unwrap();
    let y: i32 = iy.trim().parse().unwrap();
    
    let result = match op {
        '+' => calc::add::add(x, y),
        '-' => calc::sub::sub(x, y),
        '*' => calc::mul::mul(x, y),
        '/' => calc::div::div(x, y),
        'v' => calc::sqrt::sqrt(x),
        _ => panic!("Operación desconocida")
    };

    println!("Resultado: {}", result);
}
