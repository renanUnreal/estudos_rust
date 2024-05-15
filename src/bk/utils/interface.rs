use std::io::{stdout, Write};
use crossterm::{execute, terminal};

pub fn clear_screen(){
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.flush().unwrap();
}

fn header(h: &str, e : &str) {
    let separator = "#".repeat(50); // Repete "#" 50 vezes

    // Calcula o tamanho do padding para centralizar o cabeçalho
    let padding_size = (separator.len() - h.len()) / 2;

    // Formata o cabeçalho com o padding diretamente
    let formatted_header = format!("{:width$}", h, width = padding_size + h.len());

    // Imprime o cabeçalho, a escolha e o separador
    println!("{}\n{}\n{}\n{}\n", separator, formatted_header, e, separator);


}

fn opcoes_menu(opcs: &[&str]){
    
    let separator = "#".repeat(50); // Repete "#" 50 vezes
    // Imprime as opções
    for string in opcs {
        println!("{}", string);
    }

    // Imprime o separador final
    println!("\n{}\n", separator);
}

pub fn main_menu(){
    clear_screen();
    let msg: &str = "--- Menu Inicial ---";
    let e : &str = "Digite a opção desejada:";
    let _opcs : [&str; 6] = ["1-Exibir todos os Clientes", "2-Procurar Cliente", "3-Cadastrar Cliente", "4-Alterar Cliente", "5-Deletar Cliente", "6-Sair"];
    header(&msg, &e);
    opcoes_menu(&_opcs);
}

pub fn cadastrar_cliente(){
    clear_screen();
    let msg = "--- Cadastrar Cliente ---";
    let e : &str = "Preencha o campo a baixo";
    header(&msg, &e);
}