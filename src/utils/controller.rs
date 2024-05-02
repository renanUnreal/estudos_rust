use regex::Regex;
use std::fmt;
use chrono::{Local, Datelike};

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
    if get_data(&cliente.data_nascimento){
        println!("Cliente de maior");
        println!("Cliente cadastrado: \n");
        println!("{}",cliente);
        println!("\n\n#####################################");
    }
    else{
        println!("Lamento precisa ter mais de 18 anos para se cadastrar");
    }


    

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
pub fn get_data(_data: &str) -> bool {
    // Função para converter a string de data em um array de u32
    fn parse_date(date_str: &str) -> Option<[u32; 3]> {
        let parts: Vec<&str> = date_str.split('/').collect();
        if parts.len() != 3 {
            return None;
        }
        let day: u32 = parts[0].parse().ok()?;
        let month: u32 = parts[1].parse().ok()?;
        let year: u32 = parts[2].parse().ok()?;
        Some([day, month, year])
    }

    // Obter o ano atual como u32
    let current_year = Local::now().year() as u32;

    // Converter a string de data fornecida pelo usuário em um array de u32
    if let Some(data_user) = parse_date(_data) {
        // Verificar se a diferença entre os anos é maior que 18
        if (current_year - data_user[2]) > 18 {
            println!("é maior");
            true
        } else {
            false
        }
    } else {
        // Se a string de data fornecida pelo usuário for inválida, retornar false
        false
    }
}

fn parse_date(date_str: &str) -> Option<[u32; 3]> {
    // Divide a string da data em substrings usando "/"
    let parts: Vec<&str> = date_str.split('/').collect();

    // Verifica se há exatamente 3 partes na data
    if parts.len() != 3 {
        return None;
    }

    // Tenta converter as partes em números inteiros
    let day: u32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let year: u32 = parts[2].parse().ok()?;

    Some([day, month, year])
}
