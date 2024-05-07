mod repositories;
use repositories::visitante_repositories::select_from_table;
fn main() {
   // Chama a função para executar a consulta SELECT
    match select_from_table() {
        Ok(_) => println!("Consulta executada com sucesso!"),
        Err(err) => eprintln!("Erro ao executar a consulta: {}", err),
    }
}
