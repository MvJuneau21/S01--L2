use std::io;
fn preenche_arr(arr: &mut [i32], x: i32) {
    for i in 0..arr.len() {
        arr[i] = i as i32 * x;
    }
}

fn main() {
    let mut arr = [0; 10];
    let mut valor_input = String::new();
    let valor: i32;
    
    io::stdin().read_line(&mut valor_input).expect("failed to read line");
    valor = valor_input.trim().parse().unwrap();
    
    preenche_arr(&mut arr, valor);

    println!("{:?}", arr);
}