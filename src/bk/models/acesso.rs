#[derive(Debug)]
pub struct TipoAcesso {
    id: i16,
    access: char,
}

impl TipoAcesso {
    // Função para criar um novo Record
    pub fn new(id: i16, access: char) -> Self {
        TipoAcesso { id, access }
    }
    pub fn get_id(&self) -> i16{
        self.id
    }
    pub fn get_acess(&self) -> char{
        self.access
    }
    pub fn tipo_acess(&self) -> &str {
        match self.access {
            'v' => "Visitante",
            'e' => "Expositor",
            's' => "Suporte tecnico",
            'm' => "Manutenção Geral",
            _ => "Acesso nao definido",
        }
    }
}