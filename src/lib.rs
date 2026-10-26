use rusqlite::{params, Connection, Result};

pub fn fazer_login(conexao: &Connection, usuario: &str, senha: &str) -> Result<String> {
    let query = "SELECT * FROM usuarios WHERE username = ?1 AND password = ?2";
    let mut stmt = conexao.prepare(query)?;
    let resultado = stmt.query_row(params![usuario, senha], |_row| Ok(())).is_ok();
    if resultado {
        Ok("Acesso Permitido".to_string())
    } else {
        Ok("Acesso Negado".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_db() -> Connection {
        let conexao = Connection::open_in_memory().unwrap();
        conexao.execute(
            "CREATE TABLE usuarios (id INTEGER PRIMARY KEY, username TEXT NOT NULL, password TEXT NOT NULL)",
            [],
        ).unwrap();
        conexao.execute(
            "INSERT INTO usuarios (id, username, password) VALUES (1, 'admin', 'senha123')",
            [],
        ).unwrap();
        conexao
    }

    #[test]
    fn login_correto_retorna_acesso_permitido() {
        let conexao = setup_db();
        let resultado = fazer_login(&conexao, "admin", "senha123").unwrap();
        assert_eq!(resultado, "Acesso Permitido");
    }

    #[test]
    fn login_incorreto_retorna_acesso_negado() {
        let conexao = setup_db();
        let resultado = fazer_login(&conexao, "admin", "senha_errada").unwrap();
        assert_eq!(resultado, "Acesso Negado");
    }

    #[test]
    fn sql_injection_nao_burla_login() {
        let conexao = setup_db();
        let resultado = fazer_login(&conexao, "admin' --", "qualquer").unwrap();
        assert_eq!(resultado, "Acesso Negado");
    }
}
