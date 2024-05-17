use crate::models::visitante_model::Visitante;
use super::conn::conn;
// Função para executar uma consulta SELECT em uma tabela específica
pub fn select_from_table(page: i32) -> Result<(Vec<Visitante>, i32), postgres::Error> {
    // Estabelece a conexão com o banco de dados
    let mut client = conn()?; // Adicione 'mut' aqui
    
    // Define o número de itens por página
    let items_per_page = 50;

    // Executa a consulta SELECT com paginação na tabela desejada
    let offset = (page - 1) * items_per_page;
    let query = format!("SELECT * FROM visitantes OFFSET {} LIMIT {}", offset, items_per_page);
    let mut visitantes = Vec::new();
    for row in client.query(&query, &[])? {
        // Processa cada linha retornada pela consulta
        let visitante = Visitante::new(
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
            row.get(6),
            row.get(7),
            row.get(8),
        );
        visitantes.push(visitante);
    }

    // Executa a consulta para obter o total de itens na tabela
    let total_items: i64 = client.query_one("SELECT COUNT(*) FROM visitantes", &[])?.get(0);

    // Calcula o total de páginas
    let total_pages = ((total_items as f64) / (items_per_page as f64)).ceil() as i32;

    // Retorna um par contendo os visitantes recuperados e o total de páginas
    Ok((visitantes, total_pages))
}

pub fn create_visitante(cpf: &str, email: &str, nome_completo: &str, telefone: &str, empresa: &str, cargo: &str, acesso: &str, pontos: i32) -> Result<(), postgres::Error> {
    let mut client = conn()?;
    let statement = "INSERT INTO visitantes (cpf, email, nome_completo, telefone, empresa, cargo, acesso, pontos) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";
    client.execute(statement, &[&cpf, &email, &nome_completo, &telefone, &empresa, &cargo, &acesso, &pontos])?;
    Ok(())
}
pub fn update_visitante(cpf: &str, email: &str, nome_completo: &str, telefone: &str, empresa: &str, cargo: &str, acesso: &str, pontos: i32) -> Result<(), postgres::Error> {
    let mut client = conn()?;
    let statement = "UPDATE visitantes SET email = $1, nome_completo = $2, telefone = $3, empresa = $4, cargo = $5, acesso = $6, pontos = $7 WHERE cpf = $8";
    client.execute(statement, &[&email, &nome_completo, &telefone, &empresa, &cargo, &acesso, &pontos, &cpf])?;
    Ok(())
}
pub fn select_visitante_by_cpf(cpf: &str) -> Result<(), postgres::Error> {
    let mut client = conn()?;
    let statement = "SELECT * FROM visitantes WHERE cpf = $1";
    for row in client.query(statement, &[&cpf])? {
        let visitante = Visitante::new(
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
            row.get(6),
            row.get(7),
            row.get(8),
        );
        println!(
            "\n\n cpf: {}\n email: {}\n Nome completo: {}\n telefone: {}\n empresa: {}\n cargo: {}\n acesso: {}\n pontos: {}\n\n", 
            visitante.cpf(), visitante.email(), visitante.nome_completo(), visitante.telefone(), visitante.empresa(), visitante.cargo(), visitante.acesso(), visitante.pontos()
        );
    }
    Ok(())
}
pub fn delete_visitante_by_id(id: i32) -> Result<(), postgres::Error> {
    let mut client = conn()?;
    let statement = "DELETE FROM visitantes WHERE id = $1";
    client.execute(statement, &[&id])?;
    Ok(())
}

