use super::interface as ui;
use std::io::{self, Write};

fn tratar_input() -> char {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler os dados");

    let opc: char = match input.trim().chars().next() {
        Some(c) => c,
        None => {
            println!("Nenhum caracter inserido.");
            return ' ';
        }
    };
    opc
}

fn form_cliente(){
    ui::cadastrar_cliente();
 
    println!("Por favor, preencha as informações da pessoa:");

    // Solicitação das informações ao usuário
    print!("ID: ");
    io::stdout().flush().unwrap();
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Erro ao ler a entrada");
    

    print!("CPF: ");
    io::stdout().flush().unwrap();
    let mut cpf = String::new();
    io::stdin()
        .read_line(&mut cpf)
        .expect("Erro ao ler a entrada");

    print!("Nome: ");
    io::stdout().flush().unwrap();
    let mut nome = String::new();
    io::stdin()
        .read_line(&mut nome)
        .expect("Erro ao ler a entrada");

    print!("Sobrenome: ");
    io::stdout().flush().unwrap();
    let mut sobrenome = String::new();
    io::stdin()
        .read_line(&mut sobrenome)
        .expect("Erro ao ler a entrada");

    print!("Data de nascimento (formato YYYY-MM-DD): ");
    io::stdout().flush().unwrap();
    let mut data_nascimento_str = String::new();
    io::stdin()
        .read_line(&mut data_nascimento_str)
        .expect("Erro ao ler a entrada");

    print!("CEP: ");
    io::stdout().flush().unwrap();
    let mut cep = String::new();
    io::stdin()
        .read_line(&mut cep)
        .expect("Erro ao ler a entrada");

    print!("Telefone: ");
    io::stdout().flush().unwrap();
    let mut telefone = String::new();
    io::stdin()
        .read_line(&mut telefone)
        .expect("Erro ao ler a entrada");

    print!("Estado civil: ");
    io::stdout().flush().unwrap();
    let mut estado_civil = String::new();
    io::stdin()
        .read_line(&mut estado_civil)
        .expect("Erro ao ler a entrada");

    println!("\n\nCliente de cpf {} cadastrado com sucesso", cpf);
}

pub fn nav_main_menu() {
    ui::main_menu();
    print!("Aguardando...\n");

    match tratar_input() {
        '1' => form_cliente(),
        '2' => println!("Você escolheu a opção 2."),
        '3' => println!("Você escolheu a opção 3."),
        _ => {
            println!("Opção invalida, deseja tentar novamente? (s)=sim ou (n)=não");
            print!("Aguardando...\n");
            let opc2: char = tratar_input();
            if opc2 == 's' || opc2 == 'S' {
                nav_main_menu();
            } else {
                println!(
                    "Você precisa tentar novamente ou digitar uma opção válida para contnuar..."
                );
                println!("Digite algo para tentar novamente!");
                tratar_input();
                nav_main_menu();
            }
        }
    }
}
