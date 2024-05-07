use crate::models::visitante_model::Visitante;
use super::conn::conn;
// Função para executar uma consulta SELECT em uma tabela específica
pub fn select_from_table() -> Result<(), postgres::Error> {
    // Estabelece a conexão com o banco de dados
    let mut client = conn()?; // Adicione 'mut' aqui
    
    // Executa a consulta SELECT na tabela desejada
    for row in client.query("SELECT * FROM visitantes", &[])? {
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
        // Faça o que for necessário com os dados obtidos
        println!("\n\n cpf: {}\n email: {}\n Nome completo: {}\n telefone: {}\n empresa: {}\n cargo: {}\n acesso: {}\n pontos: {}\n\n", 
                visitante.cpf(), visitante.email(), visitante.nome_completo(), visitante.telefone(), visitante.empresa(),visitante.cargo(), visitante.acesso(), visitante.pontos());
    }
    
    // Retorna Ok(()) se a operação for bem-sucedida
    Ok(())
}

