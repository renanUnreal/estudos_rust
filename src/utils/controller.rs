use std::io;
use super::interface as ui;


pub fn nav_main_menu(){
    ui::main_menu();
    print!("Aguardando...\n");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler os dados");

    let opc : char = match input.trim().chars().next(){
        Some(c)=> c,
        None=>{
            println!("Nenhum caracter inserido.");
            return;
        }
    };

    println!("digitado : {}", opc);
}