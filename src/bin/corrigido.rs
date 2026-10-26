use rusqlite::{params, Connection, Result};

fn fazer_login(usuario: &str, senha: &str) -> Result<String> {
    let conexao = Connection::open("usuarios.db")?;

    // CORRIGIDO: consulta parametrizada
    let query = "SELECT * FROM usuarios WHERE username = ?1 AND password = ?2";

    let mut stmt = conexao.prepare(query)?;
    let resultado = stmt
        .query_row(params![usuario, senha], |_row| Ok(()))
        .is_ok();

    if resultado {
        Ok("Acesso Permitido".to_string())
    } else {
        Ok("Acesso Negado".to_string())
    }
}

fn main() -> Result<()> {
    let conexao = Connection::open("usuarios.db")?;
    conexao.execute(
        "CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;
    conexao.execute(
        "INSERT OR IGNORE INTO usuarios (id, username, password) VALUES (1, 'admin', 'senha123')",
        [],
    )?;

    println!("Login normal: {}", fazer_login("admin", "senha123")?);
    println!("SQLi (bypass): {}", fazer_login("admin' --", "qualquer")?);

    Ok(())
}
