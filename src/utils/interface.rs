use std::io::{stdout, Write};
use crossterm::{execute, terminal};

fn clear_screen(){
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout.flush().unwrap();
}

pub fn main_menu(){
    clear_screen();
    
    let header = "--- Menu Inicial ---";
    let escolha = "Escolha a opção desejada:";
    let separator = "#".repeat(50); // Repete "#" 50 vezes
    let opcs : [&str; 6] = ["1-Exibir todos os Clientes", "2-Procurar Cliente", "3-Cadastrar Cliente", "4-Alterar Cliente", "5-Deletar Cliente", "6-Sair"];

    // Calcula o tamanho do padding para centralizar o cabeçalho
    let padding_size = (separator.len() - header.len()) / 2;
    // Formata o cabeçalho com o padding diretamente
    let formatted_header = format!("{:width$}", header, width = padding_size + header.len());
    // Imprime o cabeçalho, a escolha e o separador
    println!("{}\n{}\n{}\n{}\n", separator, formatted_header, escolha, separator);
    for string in &opcs {
        print!("\n{}", string);
    }
    print!("\n\n{}\n", separator);

}