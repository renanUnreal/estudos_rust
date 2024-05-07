use postgres::{Client, NoTls};

// Função para estabelecer a conexão com o banco de dados
pub fn conn() -> Result<Client, postgres::Error> {
    // Altere as informações de conexão conforme necessário
    let url = "postgres://postgres:root@localhost/checkineventos";

    // Tenta estabelecer a conexão com o banco de dados
    let client = Client::connect(url, NoTls)?;

    // Retorna o cliente de banco de dados se a conexão for bem-sucedida
    Ok(client)
}
