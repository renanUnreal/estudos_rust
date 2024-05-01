use regex::Regex;
use std::fmt;

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
fn regex_form(padrao: &str, label : &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada");

    // Define a expressão regular com o padrão fornecido
    let re = Regex::new(padrao).unwrap();

    // Aplica a expressão regular na entrada
    if let Some(mat) = re.find(&input) {
        // Se a expressão regular encontrar uma correspondência na entrada
        // Retorna a parte correspondente da entrada
        mat.as_str().to_string()
    } else {
        println!("Erro ao validar dados");
        // Se não houver correspondência, retorna uma string vazia
        String::new()
    }
}

fn form_cliente(){
    ui::cadastrar_cliente();

    struct Cliente {
        id: String,
        nome_completo: String,
        cpf: String,
        civel: String,
        cep: String,
        data_nascimento: String,
    }

    let cliente: Cliente = Cliente{
    
    id: regex_form(r"\d{4}", "ID (Min 4 digitos)"),
    nome_completo : regex_form(r"^[A-Za-z\s]+$", "Nome Completo"),
     cpf : regex_form(r"^\d{11}", "CPF (11 digitos)"),
     civel : regex_form(r"^[A-Za-z\s]+$", "Estado Civil"),
     cep : regex_form(r"^\d{8}", "CEP (8 digitos)"),
     data_nascimento : regex_form(r"\d{2}\/\d{2}\/\d{4}", "Data nascimento dd/mm/yyyy"),
    };

    impl fmt::Display for Cliente {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ID: {}\nNome Completo: {}\nCPF: {}\nEstado Civil: {}\nCEP: {}\nData de Nascimento: {}",
                   self.id, self.nome_completo, self.cpf, self.civel, self.cep, self.data_nascimento)
        }
    }
    ui::clear_screen();
    println!("Cliente cadastrado: \n");
    println!("{}",cliente);
    println!("\n\n#####################################");

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
