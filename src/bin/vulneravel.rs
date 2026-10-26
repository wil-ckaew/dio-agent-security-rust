use rusqlite::{Connection, Result};

fn fazer_login(usuario: &str, senha: &str) -> Result<String> {
    let conexao = Connection::open("usuarios.db")?;

    // VULNERÁVEL: concatena entrada do usuário na string SQL
    let query = format!(
        "SELECT * FROM usuarios WHERE username = '{}' AND password = '{}'",
        usuario, senha
    );

    let mut stmt = conexao.prepare(&query)?;
    let resultado = stmt.query_row([], |_row| Ok(())).is_ok();

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
