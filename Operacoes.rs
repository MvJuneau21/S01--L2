fn main() {
  use std::io;
    let mut x_input = String::new();
    let mut y_input = String::new();
    
    let mut op = String::new();
    let res: i32;
    let x: i32;
    let y: i32;
    
    io::stdin().read_line(&mut x_input).expect("failed to read line");
    io::stdin().read_line(&mut y_input).expect("failed to read line");
    
    println!("Deseja somar ou multiplicar?");
    io::stdin().read_line(&mut op).expect("failed to read line");
    
    x = x_input.trim().parse().unwrap();
    y = y_input.trim().parse().unwrap();
    
    if op.trim() == "+"{
      res = x + y;
      println!("Soma dos numeros digitados: {}", res);
    }
    else{
      res = x*y;
      println!("Produto dos numeros digitados: {}", res);
    }
}