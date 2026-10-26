use rusqlite::{params, Connection, Result};

pub fn fazer_login(usuario: &str, senha: &str) -> Result<String> {
    let conexao = Connection::open("usuarios.db")?;
    let query = "SELECT * FROM usuarios WHERE username = ?1 AND password = ?2";
    let mut stmt = conexao.prepare(query)?;
    let resultado = stmt.query_row(params![usuario, senha], |_row| Ok(())).is_ok();
    if resultado {
        Ok("Acesso Permitido".to_string())
    } else {
        Ok("Acesso Negado".to_string())
    }
}
