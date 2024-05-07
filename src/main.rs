use postgres::{Client, NoTls};

// Função para estabelecer a conexão com o banco de dados
fn conn() -> Result<Client, postgres::Error> {
    // Altere as informações de conexão conforme necessário
    let url = "postgres://postgres:root@localhost/checkineventos";

    // Tenta estabelecer a conexão com o banco de dados
    let client = Client::connect(url, NoTls)?;

    // Retorna o cliente de banco de dados se a conexão for bem-sucedida
    Ok(client)
}

// Função para executar uma consulta SELECT em uma tabela específica
fn select_from_table() -> Result<(), postgres::Error> {
    // Estabelece a conexão com o banco de dados
    let mut client = conn()?; // Adicione 'mut' aqui

// Executa a consulta SELECT na tabela desejada
for row in client.query("SELECT * FROM visitantes", &[])? {
    // Processa cada linha retornada pela consulta
    let column1: i32 = row.get(0);
    let column2: String = row.get(1);
    let column3: String = row.get(2);
    let column4: String = row.get(3);
    let column5: String = row.get(4);
    let column6: String = row.get(5);
    let column7: String = row.get(6);
    let column8: i32 = row.get(7);
    let column9: i32 = row.get(8);

    // Faça o que for necessário com os dados obtidos
    println!("\n\nid: {}\n cpf: {}\n email: {}\n Nome completo: {}\n telefone: {}\n empresa: {}\n cargo: {}\n acesso: {}\n pontos: {}\n\n", 
            column1, column2, column3, column4, column5, column6, column7, column8, column9);
}

    // Retorna Ok(()) se a operação for bem-sucedida
    Ok(())
}

fn main() {
    // Chama a função para executar a consulta SELECT
    match select_from_table() {
        Ok(_) => println!("Consulta executada com sucesso!"),
        Err(err) => eprintln!("Erro ao executar a consulta: {}", err),
    }
}
