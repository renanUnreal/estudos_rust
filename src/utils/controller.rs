use regex::Regex;
use std::fmt;
use chrono::{Local, Datelike};
use std::io::{self, Write};

use super::interface as ui;
use super::cliente_model;

type ClienteList =  Vec<cliente_model::Cliente>;
static mut LISTA_CLIENTES: Option<ClienteList> = None;

// Inicializa a variável global LISTA_CLIENTES
fn initialize_global() {
    unsafe {
        LISTA_CLIENTES = Some(Vec::new());
    }
}

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

fn regex_form(padrao: &str, label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada");

    let re = Regex::new(padrao).unwrap();

    if let Some(mat) = re.find(&input) {
        mat.as_str().to_string()
    } else {
        println!("Erro ao validar dados");
        String::new()
    }
}

fn register_users() {
    println!("Quantos Clientes você deseja cadastrar? max: 9 \n");
    let input: char = tratar_input();
    initialize_global();

    if let Some(digit) = input.to_digit(10) {
        let num: i8 = digit as i8;

        for cliente_num in 1..=num {
          unsafe {
            if let Some(ref mut lista_clientes) = LISTA_CLIENTES {
                if let Some(cliente) = form_cliente() {
                    lista_clientes.push(cliente);
                } else {
                    println!("Falha ao cadastrar o cliente número {}", cliente_num);
                }
            }
        }
        println!("#####################################################################");
        list_clientes();
        }
    } else {
        println!("O caractere '{}' não é um dígito válido.", input);
    }
}
fn list_clientes() {
    unsafe {
        if let Some(ref lista_clientes) = LISTA_CLIENTES {
            for cliente in lista_clientes {
                println!("{}", cliente);
            }
        } else {
            println!("Lista de clientes vazia.");
        }
    }
}

fn form_cliente() -> Option<cliente_model::Cliente> {
    ui::cadastrar_cliente();
    let cliente: cliente_model::Cliente = cliente_model::Cliente {
        id: regex_form(r"\d{4}", "ID (Min 4 digitos)"),
        nome_completo: regex_form(r"^[A-Za-z\s]+$", "Nome Completo"),
        cpf: regex_form(r"^\d{11}", "CPF (11 digitos)"),
        civel: regex_form(r"^[A-Za-z\s]+$", "Estado Civil"),
        cep: regex_form(r"^\d{8}", "CEP (8 digitos)"),
        data_nascimento: regex_form(r"\d{2}\/\d{2}\/\d{4}", "Data nascimento dd/mm/yyyy"),
    };

    ui::clear_screen();
    if get_data(&cliente.data_nascimento) {
        println!("Cliente de maior");
        println!("Cliente cadastrado: \n");
        println!("{}", cliente);
        println!("\n\n#####################################");
        Some(cliente)
    } else {
        println!("Lamento, precisa ter mais de 18 anos para se cadastrar");
        None
    }
}

impl fmt::Display for cliente_model::Cliente {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}\nNome Completo: {}\nCPF: {}\nEstado Civil: {}\nCEP: {}\nData de Nascimento: {}",
               self.id, self.nome_completo, self.cpf, self.civel, self.cep, self.data_nascimento)
    }
}

pub fn nav_main_menu() {
    ui::main_menu();
    print!("Aguardando...\n");

    match tratar_input() {
        '1' => register_users(),
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

    let now = Local::now();
    let current_year = now.year() as u32;
    let current_month = now.month();
    let current_day = now.day();

    if let Some(data_user) = parse_date(_data) {
        if (current_year - data_user[2]) > 18 {
            true
        } else if (current_year - data_user[2]) == 18 {
            if current_month > data_user[1] || (current_month == data_user[1] && current_day >= data_user[0]) {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
