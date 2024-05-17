pub struct Visitante {
    cpf: String,
    email: String,
    nome_completo: String,
    telefone: String,
    empresa: String,
    cargo: String,
    acesso: i32,
    pontos: i32,
}

impl Visitante {
    // Método construtor para criar uma nova instância de Visitante
    pub fn new(
        cpf: String,
        email: String,
        nome_completo: String,
        telefone: String,
        empresa: String,
        cargo: String,
        acesso: i32,
        pontos: i32,
    ) -> Self {
        Visitante {
            cpf,
            email,
            nome_completo,
            telefone,
            empresa,
            cargo,
            acesso,
            pontos,
        }
    }

    // Métodos getters para acessar os campos da struct
    pub fn cpf(&self) -> &str {
        &self.cpf
    }

    pub fn email(&self) -> &str {
        &self.email
    }

   pub fn nome_completo(&self) -> &str {
        &self.nome_completo
    }

    pub fn telefone(&self) -> &str {
        &self.telefone
    }

    pub fn empresa(&self) -> &str {
        &self.empresa
    }

    pub fn cargo(&self) -> &str {
        &self.cargo
    }

    pub fn acesso(&self) -> i32 {
        self.acesso
    }

    pub fn pontos(&self) -> i32 {
        self.pontos
    }
}