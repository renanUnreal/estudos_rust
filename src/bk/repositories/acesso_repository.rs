use crate::models::acesso::{self, TipoAcesso};
use crate::repositories::conn::conn;

pub fn acesso_select_from_table() -> Result<(), postgres::Error> {
    let mut client = conn()?; // Adicione 'mut' aqui

    for row in client.query("SELECT * FROM tabela_acesso", &[])? {
        let id: i64 = row.get(0);
        let type_access: TipoAcesso = row.get(1);

        println!("ID: {}, Type Access: {}", id, type_access);
    }

    Ok(())
}
